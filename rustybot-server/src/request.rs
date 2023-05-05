use bytes::Bytes;
use futures::StreamExt;
use tokio::sync::mpsc::Sender;

pub async fn post_remote_stream<T>(
    endpoint: impl std::fmt::Display,
    data: &T,
    sender: Option<Sender<Bytes>>,
) -> actix_web::HttpResponse
where
    T: serde::Serialize + ?Sized,
{
    let config = rust_ai::utils::config::Config::load().unwrap();
    let url = format!("{}{}", config.openai.base_endpoint(), endpoint);
    let mut req = reqwest::Client::new().post(url);
    req = req.header("Authorization", format!("Bearer {}", config.openai.api_key));

    let res = req.json(&data).send().await.unwrap();

    let mut resp_builder = actix_web::HttpResponse::Ok();

    for header in res.headers().iter() {
        resp_builder.append_header(header);
    }

    let mut stream = res.bytes_stream();

    resp_builder.streaming(async_stream::stream! {
        while let Some(item) = stream.next().await{
            let output = match item {
                Ok(bytes) => {
                    if let Some(sender) = sender.clone() {
                        if let Err(e) = sender.send(bytes.clone()).await {
                            log::error!(target: "app", "Error extracting bytes from stream: `{e}`");
                        };
                    }
                    Ok(bytes)
                },
                Err(e) => {
                    Err(e)
                }
            };
            yield output;
        }

        if let Some(sender) = sender.clone() {
            if let Err(e) = sender.send(Bytes::from(b"EOS__EOS".to_vec())).await {
                log::error!(target: "app", "Error extracting bytes from stream: `{e}`");
            };
        }
    })
}

pub async fn post_remote<T>(
    endpoint: impl std::fmt::Display,
    data: &T,
    sender: Option<Sender<Bytes>>,
) -> actix_web::HttpResponse
where
    T: serde::Serialize + ?Sized,
{
    let config = rust_ai::utils::config::Config::load().unwrap();
    let url = format!("{}{}", config.openai.base_endpoint(), endpoint);
    let mut req = reqwest::Client::new().post(url);
    req = req.header("Authorization", format!("Bearer {}", config.openai.api_key));

    let res = req.json(&data).send().await.unwrap();

    let mut resp_builder = actix_web::HttpResponse::Ok();
    for header in res.headers().iter() {
        resp_builder.append_header(header);
    }

    let bytes = res.bytes().await.unwrap();

    if let Some(sender) = sender {
        if let Err(e) = sender.send(bytes.clone()).await {
            log::error!(target: "app", "Error extracting bytes from stream: `{e}`");
        };
    }

    resp_builder.body(bytes)
}
