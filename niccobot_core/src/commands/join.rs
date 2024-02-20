use std::num::NonZeroU64;
use std::time::Duration;
use songbird::{Event, TrackEvent};
use tracing::info;
use crate::client::{Context, Error};
use crate::commands::models::music::{ChannelDurationNotifier, TrackEndNotifier};

#[poise::command(slash_command, prefix_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            println!("Command not invoked in a guild.");
            return Ok(());
        }
    };

    let voice_states = {
        let cache = ctx.cache();
        match cache.guild(guild_id) {
            Some(guild) => guild.voice_states.clone(),
            None => {
                println!("Guild not found.");
                return Ok(());
            }
        }
    };

    let mut author_channel_id: Option<serenity::model::id::ChannelId> = None;

    for (user_id, voice_state) in voice_states.iter() {
        if *user_id == ctx.author().id {
            info!("Found User {:?} in active voice channel {:?}", ctx.author().id,voice_state.channel_id );
            author_channel_id = voice_state.channel_id

        }
    }

    if let Some(channel_id) = author_channel_id {
        info!("Author is in voice channel: {}", channel_id);
        let sb_channel_id = songbird::id::ChannelId(NonZeroU64::try_from(channel_id.get()).unwrap());
        let manager = songbird::get(ctx.serenity_context())
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();

        if let Ok(handle_lock) = manager.join(guild_id, sb_channel_id).await {
            ctx.channel_id().say(&ctx.serenity_context().http, &format!("Joined {}", sb_channel_id.0)).await?;

            let send_http = ctx.serenity_context().http.clone();
            let mut handle = handle_lock.lock().await;

            handle.add_global_event(
                Event::Track(TrackEvent::End),
                TrackEndNotifier {
                    chan_id: ctx.channel_id(),
                    http: send_http,
                },
            );

            let send_http =  ctx.serenity_context().http.clone();

            handle.add_global_event(
                Event::Periodic(Duration::from_secs(60), None),
                ChannelDurationNotifier {
                    chan_id: ctx.channel_id(),
                    count: Default::default(),
                    http: send_http,
                },
            );
        }
        else {
            ctx.channel_id().say(&ctx.serenity_context().http, &format!("Error Joining Channel {}", sb_channel_id.0)).await?;
        };

    } else {
        println!("Author is not in any voice channel.");
    }
    Ok(())
}