use std::sync::Arc;
use reqwest::Client as HttpClient;
use crate::intents::NiccoBotIntents;
extern crate dotenv;
use serenity::all::GatewayIntents;
use serenity::prelude::*;
use songbird::SerenityInit;
use poise::serenity_prelude as serenity;
use tracing::info;
use crate::commands::{ age, play };
use crate::models::http::{HttpKey};
pub struct Data {
    pub tracks: Arc<Mutex<Vec<String>>>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(derive_builder::Builder)]
pub struct Niccobot {
    token: String,
    #[builder(default = "NiccoBotIntents::default().into()")]
    intents: GatewayIntents,

}



impl Niccobot {
    #[must_use]
    pub fn builder() -> NiccobotBuilder { NiccobotBuilder::default() }

    pub async fn start(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let framework = poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands: vec![age(),
                               play(),],
                pre_command: |ctx| {
                  Box::pin(async move {
                      info!("Performing Command {}...", ctx.command().qualified_name);
                  })
                },
                post_command: |ctx| {
                    Box::pin(async move {
                        info!("Executed Command {}...", ctx.command().qualified_name);
                    })
                },
                on_error: |error| {
                    Box::pin(async move {
                        println!("what the hell");
                        match error {
                            poise::FrameworkError::ArgumentParse { error, .. } => {
                                if let Some(error) = error.downcast_ref::<serenity::RoleParseError>() {
                                    println!("Found a RoleParseError: {:?}", error);
                                } else {
                                    println!("Not a RoleParseError :(");
                                }
                            }
                            other => poise::builtins::on_error(other).await.unwrap(),
                        }
                    })
                },
                ..Default::default()
            })
            .setup(move |ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data {
                        tracks: Arc::new(Mutex::new(Vec::new())),
                    })
                })
            })
            .build();

        let mut client = Client::builder(self.token, self.intents)
            .framework(framework)
            .register_songbird()
            .type_map_insert::<HttpKey>(HttpClient::new())
            .await?;
        client
            .start()
            .await?;

        Ok(())
    }
}




