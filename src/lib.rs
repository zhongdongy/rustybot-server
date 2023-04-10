use std::sync::mpsc::channel;
use std::time::Duration;

use actix_web::{post, web, App, HttpServer};
use atc::{ChannelCommand, ServerCommand, Server};
use log::{debug, error, info};
use serde_json::{from_str, to_string};
use utils::redis::get_connection;

use crate::libs::job::JobBuilder;
use crate::models::queued::QueuedCompletionJob;
use crate::models::request::CompletionRequest;
use crate::models::response::CompletionResponse;

pub mod libs;
pub mod models;
pub mod utils;

#[post("/completion/")]
async fn completion(data: web::Json<CompletionRequest>) -> web::Json<CompletionResponse> {
    let prompts = data.prompts.clone();
    let job_id = uuid::Uuid::new_v4().hyphenated().to_string();
    let queued_job = QueuedCompletionJob {
        job_id: job_id.clone(),
        prompts: prompts,
    };

    if let Ok(mut connection) = get_connection() {
        if let Err(e) = redis::cmd("RPUSH")
            .arg("rustybot-queue:completion-request")
            .arg(to_string(&queued_job).unwrap())
            .query::<()>(&mut connection)
        {
            error!(target: "app","{}", format!(
                "Unable to send completion request to Redis: `{:?}`",
                e
            ));
            return web::Json(CompletionResponse {
                job_id: job_id,
                error: Some(format!(
                    "Unable to send completion request to Redis: `{:?}`",
                    e
                )),
            });
        }
        debug!(target: "app","Job (`{}`) submitted to Redis queue.", job_id);
        return web::Json(CompletionResponse {
            job_id: job_id,
            error: None,
        });
    }
    return web::Json(CompletionResponse {
        job_id: job_id,
        error: Some("Unable to send completion request to Redis".to_string()),
    });
}

pub async fn create_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(completion))
        .bind(("0.0.0.0", 9090))?
        .run()
        .await
}

pub fn create_job_handler() {
    let (tx_message, rx_message) = channel();

    let (tx_message_async, rx_message_async) = tokio::sync::mpsc::channel::<ServerCommand>(1024);
    


    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let _eg = rt.enter();

    tokio::spawn(async move {
        Server::new("0.0.0.0:52926".into(), rx_message_async).start().await.unwrap();
    });



    loop {

        // Try to get message from internal threads and forward to Async TCP 
        // Channel server.
        if let Ok(ChannelCommand::ChannelMessage((job_id,message))) = rx_message.try_recv() {
            let tx_message_async =tx_message_async.clone();
            tokio::spawn(async move {
                tx_message_async.send(ServerCommand::Message(Some(job_id), message)).await.unwrap();
            });
        }


        match get_connection() {
            Ok(mut connection) => {
                match redis::cmd("LPOP")
                    .arg("rustybot-queue:completion-request")
                    .query::<Option<String>>(&mut connection)
                {
                    Ok(request_contents) => {
                        if let Some(request_contents) = request_contents {
                            let sender_clone = tx_message.clone();
                            std::thread::spawn(move || {
                                let queued_job: QueuedCompletionJob =
                                  from_str(&request_contents).unwrap();
                                let job_id = queued_job.job_id.clone();
                                let prompts = queued_job.prompts.clone();
                                std::thread::sleep(Duration::from_secs(2));
                                let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
                                rt.block_on(async move {
                                    info!(target: "app", "Ready to execute one completion job");
                                    match JobBuilder::new(job_id, sender_clone.clone())
                                        .set_messages(prompts)
                                        .finalize()
                                        .completion()
                                        .await
                                    {
                                      Err(e) => 
                                      error!(target: "app", "Error happended when executing completion job `{}`: {:?}", queued_job.job_id, e),
                                      Ok(_)=>
                                      info!(target:"app", "Completion job `{}` finished.", queued_job.job_id)
                                    };
                                });
                            });
                        } else {
                            std::thread::sleep(Duration::from_millis(500));
                        }
                    }
                    Err(cmd_e) => {
                        error!(target: "app","Unable to retrieve job from Redis queue: {:?}", cmd_e);
                    }
                }
            }
            Err(e) => {
                error!(target: "app", "Unable to connect to Redis: {:?}", e);
            }
        };
    }
}
