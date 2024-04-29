use poise::serenity_prelude as serenity;
use crate::client::{Context, Error};
use sqlx::sqlite::SqlitePool;
use sqlx::{Row};
use base64::{engine::general_purpose, Engine as _};
use tracing::info;
use serde_json::{Map, to_string, Value};
use serenity::all::{CreateEmbed, Message, MessageId, Role};
use serenity::builder::CreateMessage;
use tokio::time::{sleep, Duration};
use tracing::log::error;
use niccobot_util::crypto::{encrypt, decrypt, generate_secure_salt, hash_string};


// #[poise::command(slash_command, prefix_command)]
// pub async fn get_smurfs(
//     ctx: Context<'_>,
//     #[description = "Discord user to check"] user: Option<serenity::User>,
// ) -> Result<(), Error> {
//     let user = user.as_ref().unwrap_or_else(|| ctx.author());
//     let db_pool = &ctx.data().db.pool;
//     let smurfs_exist = "SELECT EXISTS(SELECT 1 FROM smurfs)";
//
//     let (exists,): (bool,) = sqlx::query_as(smurfs_exist)
//         .fetch_one(db_pool)
//         .await?;
//
//     if exists {
//         info!("User {:?} exists in db retrieving data", &user.name);
//         let platforms_query = "SELECT platforms FROM game_ids WHERE discord_username = ?";
//         let row = sqlx::query(platforms_query)
//             .bind(&user.name)
//             .fetch_one(db_pool)
//             .await?;
//
//         let platform_data: String = row.try_get("platforms")
//             .unwrap_or_else(|_| "{}".to_string());
//         let platforms: Value = serde_json::from_str(&platform_data)?;
//         let platform_object = platforms.as_object().unwrap();
//         let mut pretty_output = String::new();
//         for (key,value) in platform_object {
//             pretty_output.push_str(&format!("**{}**: {}\n", key, value))
//         }
//         ctx.say(pretty_output).await?;
//     }
//     else {
//         info!("User {:?} does not exists in db", &user.name);
//         ctx.say(format!("User '{}' does not have any account details to \
//                              share, he is a bit of a loner", &user.name)).await?;
//     }
//
//     Ok(())
// }
//
// #[poise::command(slash_command, prefix_command)]
// pub async fn add_accounts(
//     ctx: Context<'_>,
//     #[description = "Discord user to check"] user: Option<serenity::User>,
//     #[description = "Platform name"] platform: String,
//     #[description = "Username for platform"] platform_username: String,
// ) -> Result<(), Error> {
//     let user = user.as_ref().unwrap_or_else(|| ctx.author());
//     let db_pool = &ctx.data().db.pool;
//     let user_exists_query = "SELECT EXISTS(SELECT 1 FROM game_ids WHERE discord_username = ?)";
//
//     let (user_exists,): (bool,) = sqlx::query_as(user_exists_query)
//         .bind(&user.name)
//         .fetch_one(db_pool)
//         .await?;
//
//     if user_exists {
//         info!("User {:?} exists in db retrieving data", &user.name);
//         let validated_platform_name = validate_platform(platform);
//         match check_platform_exists(&validated_platform_name, db_pool, &user.name).await {
//             Ok(Some(mut platform)) => {
//                 info!("Platform exists already");
//                 let platform_object = platform.as_object_mut().unwrap();
//                 platform_object.insert(validated_platform_name, Value::String(platform_username.clone()));
//                 match update_platforms_in_db(db_pool, &user.name, platform_object).await {
//                     Ok(()) => {
//                         ctx.say("Added new game id details to account :)").await?;
//                     }
//                     Err(e) => {
//                         error!("Error occured {:?}", e);
//                         ctx.say("An error occured while trying to add your details please contact the bot administrator").await?;
//                     }
//                 }
//             }
//             Ok(None) => {
//                 info!("Platform does not exist in table columns adding...");
//                 let alter_query = "SELECT * FROM game_ids WHERE discord_username = ?";
//
//                 let _result = sqlx::query(alter_query)
//                     .bind(&validated_platform_name)
//                     .execute(db_pool)
//                     .await
//                     .unwrap();
//
//             }
//             Err(e) => error!("An error occurred: {}", e)
//         }
//     }
//     else {
//         info!("User {:?} does not exists in db creating empty entry", &user.name);
//         let insert_user_query = "INSERT INTO game_ids (discord_username) VALUES (?)";
//
//         sqlx::query(insert_user_query)
//             .bind(&user.name)
//             .execute(db_pool)
//             .await?;
//         info!("User {} added to the database. Adding platform details", &user.name);
//         let validated_platform_name = validate_platform(platform);
//         match check_platform_exists(&validated_platform_name, db_pool, &user.name).await {
//             Ok(Some(mut platform)) => {
//                 info!("Platform data exists already");
//                 let platform_object = platform.as_object_mut().unwrap();
//                 platform_object.insert(validated_platform_name, Value::String(platform_username.clone()));
//                 match update_platforms_in_db(db_pool, &user.name, platform_object).await {
//                     Ok(()) => {
//                         ctx.say("Added new game id details to account :)").await?;
//                     }
//                     Err(e) => {
//                         error!("Error occured {:?}", e);
//                         ctx.say("An error occured while trying to add your details please contact the bot administrator").await?;
//                     }
//                 }
//             }
//             Ok(None) => {
//                 info!("Platform does not exist in table columns adding...");
//                 let alter_query = "SELECT * FROM game_ids WHERE discord_username = ?";
//                 let _platforms = {};
//
//                 let _result = sqlx::query(alter_query)
//                     .bind(&validated_platform_name)
//                     .execute(db_pool)
//                     .await
//                     .unwrap();
//
//             }
//             Err(e) => error!("An error occurred: {}", e)
//         }
//     }
//
//     Ok(())
// }

#[poise::command(slash_command, prefix_command)]
pub async fn get_key(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or_else(|| Error::from("This command must be executed in a guild."))?;
    let roles_to_check = if let Some(guild) = ctx.guild() {
        println!("Guild ID where the command was executed: {}", &guild.id);
        let guild_roles_data = &guild.roles;
        guild_roles_data
            .iter()
            .filter(|(roleId, role)| role.guild_id == guild_id && role.name == "Verified Smurf")
            .map(|(id, data)| (id.clone(), data.clone()))
            .collect::<Vec<(poise::serenity_prelude::RoleId, Role)>>()
        
    }
    else { 
        return Err(Error::from("Guild Context is unavailable"))
    };
    
    let mut has_verified_role = false;
    for (roleId, role) in roles_to_check {
        let user_roles = match ctx.author().has_role(&ctx.serenity_context().http,&guild_id, &roleId).await {
            Ok(has_role) => {
                info!("User has the necessary roles 'Verified Smurf' to proceed with command");
                has_verified_role = true;
                break;
            }
            Err(_) => return Err(Error::from("Failed to fetch user roles"))
        };
    }
    
    if has_verified_role { 
        info!("Retrieving key from database");
        let auth_key_exists_query = "SELECT auth_key FROM auth_keys WHERE table_name = 'smurfs'";
        let row = sqlx::query(auth_key_exists_query)
            .fetch_optional(&ctx.data().db.pool)
            .await?;
        match row {
            Some(row) => {
                let auth_key: String = row.try_get("auth_key").unwrap();
                info!("{:?}", auth_key);
                let dm_channel = &ctx.author().create_dm_channel(&ctx.http()).await.unwrap();
                let dm_embed = CreateEmbed::new().title("Authorization Key").description("This is the key used to get your passwords.\
                This message will self-destruct in 30 seconds")
                    .field("Key", auth_key, false);
                let dm_message = &dm_channel.send_message(&ctx.http(), CreateMessage::new().embed(dm_embed)).await;

                &ctx.say("Check your messages I have sent you your access token").await;

                match dm_message {
                    Ok(message) => {
                        let account_details_message_id = message.id;

                        sleep(Duration::from_secs(30)).await;
                        let mut message_vec: Vec<MessageId> = Vec::new();
                        message_vec.push(account_details_message_id);
                        let dm_deleted = dm_channel.delete_messages(&ctx.http(), message_vec).await;
                        match dm_deleted {
                            Ok(()) => {
                                info!("Message with account details removed after timer.");
                            }
                            Err(e) => {
                                error!("Failed to delete account details message...{:?}", e)
                            }
                        }
                    }
                    Err(e) => {
                        return Err(Error::from("Message could not be found to delete"))
                    }
                }

            }
            None => {
                return Err(Error::from("auth_key does not exist"))
            }
        }
    }
    else {
        return Err(Error::from("User does not have the necessary roles to run this command"))
    }
    
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn add_smurf(
    ctx: Context<'_>,
    #[description = "Username for the account"] username: String,
    #[description = "Platform name"] platform: String,
    #[description = "Password for account"] password: String,
    #[description = "Extra account info"] info: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or_else(|| Error::from("This command must be executed in a guild."))?;
    let roles_to_check = if let Some(guild) = ctx.guild() {
        println!("Guild ID where the command was executed: {}", &guild.id);
        let guild_roles_data = &guild.roles;
        guild_roles_data
            .iter()
            .filter(|(roleId, role)| role.guild_id == guild_id && role.name == "Verified Smurf")
            .map(|(id, data)| (id.clone(), data.clone()))
            .collect::<Vec<(poise::serenity_prelude::RoleId, Role)>>()
    }
    else {
        return Err(Error::from("Guild Context is unavailable"))
    };

    let mut has_verified_role = false;
    for (roleId, role) in roles_to_check {
        let user_roles = match ctx.author().has_role(&ctx.serenity_context().http,&guild_id, &roleId).await {
            Ok(has_role) => {
                info!("User has the necessary roles 'Verified Smurf' to proceed with command");
                has_verified_role = true;
                break;
            }
            Err(_) => return Err(Error::from("Failed to fetch user roles"))
        };
    }

    if has_verified_role {
        let secret: &[u8; 13] = b"$m1tHc!@n2022";
        let salt = generate_secure_salt();
        let key = hash_string(secret, &salt);

        match key {
            Ok(key) => {
                match encrypt(&password, &key) {
                    Ok((ciphertext, nonce)) => {
                        info!("Successfully encrypted password beginning write sequence to database");
                        println!("Ciphertext: {:?}", ciphertext);
                        println!("Nonce: {:?}", nonce);
                        sqlx::query("INSERT INTO smurfs (account_name, salt, nonce, password, platform) VALUES (?, ?, ?, ?, ?)")
                            .bind(username)
                            .bind(general_purpose::STANDARD.encode(&salt))
                            .bind(general_purpose::STANDARD.encode(&nonce))
                            .bind(general_purpose::STANDARD.encode(&ciphertext))
                            .bind(platform)
                            .execute(&ctx.data().db.pool)
                            .await
                            .expect("Failed to insert smurf data into the database");
                        
                        sqlx::query("INSERT INTO auth_keys (auth_key, table_name) VALUES (?, ?)")
                            .bind(general_purpose::STANDARD.encode(&key))
                            .bind("smurfs")
                            .execute(&ctx.data().db.pool)
                            .await
                            .expect("Failed to insert into auth_keys");
                        

                        info!("Successfully encrypted password and stored in the database");
                        
                        // match decrypt(&ciphertext, &nonce, &key) {
                        //     Ok(plaintext) => println!("Decrypted text: {}", plaintext),
                        //     Err(e) => println!("Decryption failed: {:?}", e),
                        // }
                    }
                    Err(e) => {
                        error!("Something went wrong in encryption process: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("Error generating hashed key for encryption");
            }
        }

    }
    else {
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn get_smurf_list(
    ctx: Context<'_>,
    #[description = "username for smurf accounts platform"] platform_filter: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or_else(|| Error::from("This command must be executed in a guild."))?;
    let roles_to_check = if let Some(guild) = ctx.guild() {
        println!("Guild ID where the command was executed: {}", &guild.id);
        let guild_roles_data = &guild.roles;
        guild_roles_data
            .iter()
            .filter(|(roleId, role)| role.guild_id == guild_id && role.name == "Verified Smurf")
            .map(|(id, data)| (id.clone(), data.clone()))
            .collect::<Vec<(poise::serenity_prelude::RoleId, Role)>>()

    }
    else {
        return Err(Error::from("Guild Context is unavailable"))
    };

    let mut has_verified_role = false;
    for (roleId, role) in roles_to_check {
        let user_roles = match ctx.author().has_role(&ctx.serenity_context().http,&guild_id, &roleId).await {
            Ok(has_role) => {
                info!("User has the necessary roles 'Verified Smurf' to proceed with command");
                has_verified_role = true;
                break;
            }
            Err(_) => return Err(Error::from("Failed to fetch user roles"))
        };
    }

    if has_verified_role {
        let get_account_query = "SELECT * FROM smurfs";
        let smurf_exists = sqlx::query(get_account_query)
            .fetch_all(&ctx.data().db.pool)
            .await?;
        

    }
    else {
        return Err(Error::from("User does not have the necessary roles to run this command"))
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn get_smurf_info(
    ctx: Context<'_>,
    #[description = "username for smurf accounts platform"] username: String,
    #[description = "Key used for decryption of password"] key: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or_else(|| Error::from("This command must be executed in a guild."))?;
    let roles_to_check = if let Some(guild) = ctx.guild() {
        println!("Guild ID where the command was executed: {}", &guild.id);
        let guild_roles_data = &guild.roles;
        guild_roles_data
            .iter()
            .filter(|(roleId, role)| role.guild_id == guild_id && role.name == "Verified Smurf")
            .map(|(id, data)| (id.clone(), data.clone()))
            .collect::<Vec<(poise::serenity_prelude::RoleId, Role)>>()

    }
    else {
        return Err(Error::from("Guild Context is unavailable"))
    };

    let mut has_verified_role = false;
    for (roleId, role) in roles_to_check {
        let user_roles = match ctx.author().has_role(&ctx.serenity_context().http,&guild_id, &roleId).await {
            Ok(has_role) => {
                info!("User has the necessary roles 'Verified Smurf' to proceed with command");
                has_verified_role = true;
                break;
            }
            Err(_) => return Err(Error::from("Failed to fetch user roles"))
        };
    }

    if has_verified_role {
        info!("Retrieving key from database");


        let auth_key_encoded: String = key;

        let auth_key_vec = general_purpose::STANDARD.decode(&auth_key_encoded).map_err(|_| "Base64 decode error".to_string());

        let auth_key: [u8; 32] = auth_key_vec.as_slice().try_into().unwrap();
        let get_account_query = "SELECT * FROM smurfs WHERE account_name = ?";
        let smurf_exists = sqlx::query(get_account_query)
            .bind(&username)
            .fetch_optional(&ctx.data().db.pool)
            .await?;

        match smurf_exists {
            Some(row) => {
                let nonce_result = row.try_get::<String, _>("nonce")
                    .map_err(|e| e.to_string())
                    .and_then(|nonce_str| {
                        general_purpose::STANDARD.decode(nonce_str)
                            .map_err(|_| "Base64 decode error".to_string())
                    }).unwrap();

                let salt_result = row.try_get::<String, _>("salt")
                    .map_err(|e| e.to_string())
                    .and_then(|salt_str| {
                        general_purpose::STANDARD.decode(salt_str)
                            .map_err(|_| "Base64 decode error".to_string())
                    }).unwrap();

                let ciphertext_result = row.try_get::<String, _>("password")
                    .map_err(|e| e.to_string())
                    .and_then(|password_str| {
                        general_purpose::STANDARD.decode(password_str)
                            .map_err(|_| "Base64 decode error".to_string())
                    }).unwrap();


                let nonce: [u8; 12] = nonce_result.as_slice().try_into()
                    .map_err(|_| "Nonce conversion error: Incorrect vector length")?;
                let salt: [u8; 16] = salt_result.as_slice().try_into()
                    .map_err(|_| "Salt conversion error: Incorrect vector length")?;

                let result = decrypt(&ciphertext_result, &nonce, &auth_key);
                let dm_channel = &ctx.author().create_dm_channel(&ctx.http()).await.unwrap();
                let dm_embed = CreateEmbed::new().title("Smurf Account details")
                    .field("Account Name: ", &username, false)
                    .field("Account Platform: ", &row.try_get::<String, _>("platform").unwrap(), false)
                    .field("Account Password: ", result.unwrap(), false);
                let dm_message = dm_channel.send_message(&ctx.http(), CreateMessage::new().embed(dm_embed)).await;
                ctx.say("The account details were sent to you in your direct messages.").await?;
                let account_details_message_id = dm_message.unwrap().id;

                sleep(Duration::from_secs(30)).await;
                let mut message_vec: Vec<MessageId> = Vec::new();
                message_vec.push(account_details_message_id);
                let dm_deleted = dm_channel.delete_messages(&ctx.http(), message_vec).await;
                match dm_deleted {
                    Ok(()) => {
                        info!("Message with account details removed after timer.");
                    }
                    Err(e) => {
                        error!("Failed to delete account details message...{:?}", e)
                    }
                }

            }
            None => {
                return Err(Error::from("Smurf account does not exist"))
            }
        }

    }
    else {
        return Err(Error::from("User does not have the necessary roles to run this command"))
    }

    Ok(())
}



// pub fn validate_platform(mut platform: String) -> String {
//     if platform.to_lowercase().contains("_id") {
//         platform.to_lowercase()
//     }
//     else {
//         platform.push_str("_id");
//         platform.to_lowercase()
//     }
// }


// pub async fn update_platforms_in_db(db_pool: &SqlitePool,
//                                     username: &str,
//                                     platforms: &mut Map<String, Value>) -> Result<(), anyhow::Error> {
//     let platforms_str = to_string(platforms)?;
//
//     let update_query = "UPDATE game_ids SET platforms = ? WHERE discord_username = ?";
//
//     let result = sqlx::query(update_query)
//         .bind(platforms_str)
//         .bind(username)
//         .execute(db_pool)
//         .await?;
//     Ok(())
// }
//
// pub async fn check_platform_exists(
//     _platform: &str,
//     db_pool: &SqlitePool,
//     username: &str
// ) -> Result<Option<Value>, anyhow::Error> {
//     let query = "SELECT platforms FROM game_ids WHERE discord_username = ?";
//
//     if let Some(row) = sqlx::query(query)
//         .bind(username)
//         .fetch_optional(db_pool)
//         .await? {
//         let platform_data: String = row.try_get("platforms")
//             .unwrap_or_else(|_| "{}".to_string());
//         let platforms: Value = serde_json::from_str(&platform_data)?;
//         Ok(Some(platforms))
//     }
//     else {
//         Ok(None)
//     }
// }