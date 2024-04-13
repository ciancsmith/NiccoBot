use poise::serenity_prelude as serenity;
use crate::client::{Context, Error};
use sqlx::sqlite::SqlitePool;
use sqlx::{Row};

use tracing::info;
use serde_json::{Map, to_string, Value};
use tracing::log::error;


#[poise::command(slash_command, prefix_command)]
pub async fn get_accounts(
    ctx: Context<'_>,
    #[description = "Discord user to check"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let db_pool = &ctx.data().db.pool;
    let user_exists_query = "SELECT EXISTS(SELECT 1 FROM game_ids WHERE discord_username = ?)";

    let (user_exists,): (bool,) = sqlx::query_as(user_exists_query)
        .bind(&user.name)
        .fetch_one(db_pool)
        .await?;

    if user_exists {
        info!("User {:?} exists in db retrieving data", &user.name);
        let platforms_query = "SELECT platforms FROM game_ids WHERE discord_username = ?";
        let row = sqlx::query(platforms_query)
            .bind(&user.name)
            .fetch_one(db_pool)
            .await?;

        let platform_data: String = row.try_get("platforms")
            .unwrap_or_else(|_| "{}".to_string());
        let platforms: Value = serde_json::from_str(&platform_data)?;
        let platform_object = platforms.as_object().unwrap();
        let mut pretty_output = String::new();
        for (key,value) in platform_object {
            pretty_output.push_str(&format!("**{}**: {}\n", key, value))
        }
        ctx.say(pretty_output).await?;
    }
    else {
        info!("User {:?} does not exists in db", &user.name);
        ctx.say(format!("User '{}' does not have any account details to \
                             share, he is a bit of a loner", &user.name)).await?;
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn add_accounts(
    ctx: Context<'_>,
    #[description = "Discord user to check"] user: Option<serenity::User>,
    #[description = "Platform name"] platform: String,
    #[description = "Username for platform"] platform_username: String,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let db_pool = &ctx.data().db.pool;
    let user_exists_query = "SELECT EXISTS(SELECT 1 FROM game_ids WHERE discord_username = ?)";

    let (user_exists,): (bool,) = sqlx::query_as(user_exists_query)
        .bind(&user.name)
        .fetch_one(db_pool)
        .await?;

    if user_exists {
        info!("User {:?} exists in db retrieving data", &user.name);
        let validated_platform_name = validate_platform(platform);
        match check_platform_exists(&validated_platform_name, db_pool, &user.name).await {
            Ok(Some(mut platform)) => {
                info!("Platform exists already");
                let platform_object = platform.as_object_mut().unwrap();
                platform_object.insert(validated_platform_name, Value::String(platform_username.clone()));
                match update_platforms_in_db(db_pool, &user.name, platform_object).await {
                    Ok(()) => {
                        ctx.say("Added new game id details to account :)").await?;
                    }
                    Err(e) => {
                        error!("Error occured {:?}", e);
                        ctx.say("An error occured while trying to add your details please contact the bot administrator").await?;
                    }
                }
            }
            Ok(None) => {
                info!("Platform does not exist in table columns adding...");
                let alter_query = "SELECT * FROM game_ids WHERE discord_username = ?";

                let _result = sqlx::query(alter_query)
                    .bind(&validated_platform_name)
                    .execute(db_pool)
                    .await
                    .unwrap();

            }
            Err(e) => error!("An error occurred: {}", e)
        }
    }
    else {
        info!("User {:?} does not exists in db creating empty entry", &user.name);
        let insert_user_query = "INSERT INTO game_ids (discord_username) VALUES (?)";

        sqlx::query(insert_user_query)
            .bind(&user.name)
            .execute(db_pool)
            .await?;
        info!("User {} added to the database. Adding platform details", &user.name);
        let validated_platform_name = validate_platform(platform);
        match check_platform_exists(&validated_platform_name, db_pool, &user.name).await {
            Ok(Some(mut platform)) => {
                info!("Platform data exists already");
                let platform_object = platform.as_object_mut().unwrap();
                platform_object.insert(validated_platform_name, Value::String(platform_username.clone()));
                match update_platforms_in_db(db_pool, &user.name, platform_object).await { 
                    Ok(()) => {
                        ctx.say("Added new game id details to account :)").await?;
                    }
                    Err(e) => {
                        error!("Error occured {:?}", e);
                        ctx.say("An error occured while trying to add your details please contact the bot administrator").await?;
                    }
                }
            }
            Ok(None) => {
                info!("Platform does not exist in table columns adding...");
                let alter_query = "SELECT * FROM game_ids WHERE discord_username = ?";
                let _platforms = {};

                let _result = sqlx::query(alter_query)
                    .bind(&validated_platform_name)
                    .execute(db_pool)
                    .await
                    .unwrap();

            }
            Err(e) => error!("An error occurred: {}", e)
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn smurfs(
    ctx: Context<'_>,
    #[description = "Smurf account details"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}


pub fn validate_platform(mut platform: String) -> String {
    if platform.to_lowercase().contains("_id") {
        platform.to_lowercase()
    }
    else {
        platform.push_str("_id");
        platform.to_lowercase()
    }
}


pub async fn update_platforms_in_db(db_pool: &SqlitePool,
                                    username: &str,
                                    platforms: &mut Map<String, Value>) -> Result<(), anyhow::Error> {
    let platforms_str = to_string(platforms)?;
    
    let update_query = "UPDATE game_ids SET platforms = ? WHERE discord_username = ?";
    
    let result = sqlx::query(update_query)
        .bind(platforms_str)
        .bind(username)
        .execute(db_pool)
        .await?;
    Ok(())
}

pub async fn check_platform_exists(
    _platform: &str,
    db_pool: &SqlitePool,
    username: &str
) -> Result<Option<Value>, anyhow::Error> {
    let query = "SELECT platforms FROM game_ids WHERE discord_username = ?";
    
    if let Some(row) = sqlx::query(query)
        .bind(username)
        .fetch_optional(db_pool)
        .await? {
        let platform_data: String = row.try_get("platforms")
            .unwrap_or_else(|_| "{}".to_string());
        let platforms: Value = serde_json::from_str(&platform_data)?;
        Ok(Some(platforms))
    }
    else { 
        Ok(None)
    }
}