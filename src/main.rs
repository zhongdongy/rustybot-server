use std::error::Error;

use rustybot_server::{create_job_handler, create_server};

fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _eg = rt.enter();

    std::thread::spawn(|| {
        create_job_handler()
    });
    // rt.spawn(async {
    //     create_job_handler().await;
    // });

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        create_server().await.unwrap();
    });

    Ok(())
}
