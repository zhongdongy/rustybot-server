use rustybot_server::{
    create_server,
    // models::{Chat, Message, MessageSender},
    utils::db::init_pool,
};
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // Must call this to initialize database pool.
    init_pool();

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

    rt.shutdown_background();

    // tokio::runtime::Runtime::new().unwrap().block_on(async {
    // let mut user = User::find_by_name("admin").await.unwrap().unwrap();
    // println!("User: {:?}", user);
    // let auth = user.auth().await;
    // // let auth = Auth::auth("admin").await.unwrap();
    // println!("Auth: {:?}", auth);
    // user = user.set_display_name("Biubiubiu");
    // println!("Value: {}", user.save().await.unwrap());
    // let mut chat = Chat::new(1).save().await.unwrap();

    // println!("Chat: {:?}", chat);

    // println!(
    //     "Chats by user 1: {:#?}",
    //     Chat::find_chats_by_user(1).await.unwrap()
    // );
    // let msg = Message::new(2, MessageSender::User, String::from("Hello!"), None)
    //     .save()
    //     .await
    //     .unwrap();
    // println!("Message: {:?}", msg);
    // println!(
    //     "Messages: {:#?}",
    //     Chat::chat_by_id(2).await.unwrap().unwrap().history().await
    // );
    // });

    Ok(())
}
