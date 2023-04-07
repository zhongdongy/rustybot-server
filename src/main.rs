use std::error::Error;

use rustybot_server::{create_job_handler, create_server};

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _eg = rt.enter();

    rt.spawn(async {
        create_job_handler().await;
    });

    rt.block_on(async {
        create_server().await.unwrap();
    });

    Ok(())
}
