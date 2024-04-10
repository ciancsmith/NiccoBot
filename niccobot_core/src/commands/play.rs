use std::num::NonZeroU64;
use std::sync::Arc;
use std::time::Duration;
use crate::client::{Context, Data, Error};
use reqwest::Client as HttpClient;
use tracing::{error, info};
use poise::serenity_prelude as serenity;
use songbird::{
    input::YoutubeDl,
    Event,
    EventContext,
    SerenityInit,
    TrackEvent,
};
use songbird::error::PlayError;
use songbird::input::{AuxMetadata, Compose, Input};
use crate::commands::models::music::{ChannelDurationNotifier, TrackEndNotifier};


struct HttpKey;

impl serenity::prelude::TypeMapKey for HttpKey {
    type Value = HttpClient;
}

pub struct TrackMetaKey;

impl serenity::prelude::TypeMapKey for TrackMetaKey {
    type Value = AuxMetadata;
}

#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[rest]
    #[description = "Song to play"]
    track: String,
) -> Result<(), Error> {
    // Extract guild_id
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            println!("Command not invoked in a guild.");
            return Ok(());
        }
    };

    // Clone the necessary data (voice_states) before the await point
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
            let mut handle = handle_lock.lock().await;
            let send_http = ctx.serenity_context().http.clone();
            let http_client = get_http_client(ctx).await;
            let mut src =  YoutubeDl::new(http_client, track);
            
            let mut input: Input = src.into();
            let metadata = input.aux_metadata().await?;
            
            let track_handle = handle.enqueue_input(input).await;

            track_handle
                .typemap()
                .write()
                .await
                .insert::<TrackMetaKey>(metadata.clone());
            
            
            ctx.channel_id().say(&ctx.serenity_context().http, "Playing Song").await?;


            handle.add_global_event(
                Event::Track(TrackEvent::End),
                TrackEndNotifier {
                    chan_id: ctx.channel_id(),
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

async fn get_http_client<'a>(ctx: poise::Context<'a, Data, Error>) -> HttpClient {
    let ctx = ctx.serenity_context();
    let data = ctx.data.read().await;
    data.get::<crate::models::http::HttpKey>()
        .cloned()
        .expect("Guaranteed to exist in the typemap.")
}