extern crate log;
extern crate dotenv;

use niccobot_core::{ Niccobot };
use log::{error, info};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let env = env_logger::Env::default().filter_or("RUST_LOG", "niccobot=debug");
    env_logger::init_from_env(env);
    info!("Starting bot");
    let niccobot = Niccobot::builder()
        .with_database("sqlite://database.sqlite")
        .await
        .unwrap()
        .token(std::env::var("DISCORD_TOKEN").expect("Discord token to be present"))
        .build()
        .expect("To build Niccobot");

    if let Err(why) = niccobot.start().await {
        error!("Client error: {:?}", why);
    }
}