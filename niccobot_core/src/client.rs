
use std::sync::Arc;
use reqwest::Client as HttpClient;
use crate::intents::NiccoBotIntents;
extern crate dotenv;
use serenity::all::GatewayIntents;
use serenity::prelude::*;
use poise::serenity_prelude as serenity;
use tracing::info;
use crate::commands::{age, get_accounts, add_accounts, get_key, add_smurf, get_smurf_info};
use crate::db::db::DB;
use crate::models::http::{HttpKey};

pub struct Data {
    pub tracks: Arc<Mutex<Vec<String>>>,
    pub db: Arc<DB>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(derive_builder::Builder)]
pub struct Niccobot {
    token: String,
    #[builder(default = "NiccoBotIntents::default().into()")]
    intents: GatewayIntents,
    db: Arc<DB>,
}

impl Niccobot {
    #[must_use]
    pub fn builder() -> NiccobotBuilder { NiccobotBuilder::default() }

    pub async fn start(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let framework = poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands: vec![age(),
                               get_accounts(), 
                               add_accounts(),
                               get_key(), 
                               add_smurf(),
                               get_smurf_info()],
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
                        db: self.db.clone(),
                    })
                })
            })
            .build();

        let mut client = Client::builder(self.token, self.intents)
            .framework(framework)
            .type_map_insert::<HttpKey>(HttpClient::new())
            .await?;

        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));

        tokio::spawn(async move {
            let _ = client
                .start()
                .await
                .map_err(|why| println!("Client ended: {:?}", why));
        });
        Ok(())
    }
}

impl NiccobotBuilder {
    pub async fn with_database(mut self, db_url: String) -> Result<Self, sqlx::Error> {
        let database: DB = DB::new(&db_url, None).await;
        database.send_migrations().await;
        self.db = Option::from(Arc::new(database));
        Ok(self)
    }
}




