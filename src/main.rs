use std::{error::Error, path::PathBuf};

use rustybot_server::create_server;

fn main() -> Result<(), Box<dyn Error>> {
    if PathBuf::from("log4rs.yml").exists() {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "1");
        log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _eg = rt.enter();

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        create_server().await.unwrap();
    });

    Ok(())
}
