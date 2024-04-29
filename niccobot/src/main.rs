extern crate log;
extern crate dotenv;

use niccobot_core::{ Niccobot };
use log::{error, info};
use dotenv::dotenv;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn readiness_probe() -> impl Responder {
    HttpResponse::Ok().body("Ready")
}

async fn liveness_probe() -> impl Responder {
    HttpResponse::Ok().body("healthy")
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let env = env_logger::Env::default().filter_or("RUST_LOG", "niccobot=debug");
    env_logger::init_from_env(env);

    let database_url = std::env::var("SQL_DB").unwrap_or("sqlite://database.sqlite".to_string());

    info!("Starting bot");
    let niccobot = Niccobot::builder()
        .with_database(database_url)
        .await
        .unwrap()
        .token(std::env::var("DISCORD_TOKEN").expect("Discord token to be present"))
        .build()
        .expect("To build Niccobot");

    let server = HttpServer::new(|| {
        App::new()
            .route("/ready", web::get().to(readiness_probe))
            .route("/health", web::get().to(liveness_probe))
    })
        .bind("0.0.0.0:8080").expect("Port and host could not be set check your configuration for these")
        .run();
    
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Failed to install CTRL+C signal handler");
    };

    // Run server, bot, and listen for the Ctrl+C signal concurrently
    tokio::select! {
        _ = server => info!("HTTP Server has stopped"),
        _ = niccobot.start() => info!("Bot has stopped"),
        _ = ctrl_c => info!("Received Ctrl+C, shutting down"),
    }
}