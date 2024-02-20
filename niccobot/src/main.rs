extern crate log;
use niccobot_core::{ Niccobot };
use log::{error, info};

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "niccobot=info");
    env_logger::init_from_env(env);


    info!("Starting bot");
    let niccobot = Niccobot::builder()
        .token(std::env::var("DISCORD_TOKEN").expect("Discord token to be present"))
        .build()
        .expect("To build Niccobot");

    if let Err(why) = niccobot.start().await {
        error!("Client error: {:?}", why);
    }
}