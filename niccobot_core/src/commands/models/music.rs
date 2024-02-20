use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use async_trait::async_trait;
use serenity::all::Http;
use songbird::{Event,
               EventContext,
               EventHandler as VoiceEventHandler,
               SerenityInit,
               TrackEvent,};

pub struct TrackEndNotifier {
    pub chan_id: serenity::model::id::ChannelId,
    pub http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
     async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            self.chan_id
                .say(&self.http, &format!("Tracks ended: {}.", track_list.len()))
                .await
                .expect("Error sending message");
        }

        None
    }
}

pub struct ChannelDurationNotifier {
    pub chan_id: serenity::model::id::ChannelId,
    pub count: Arc<AtomicUsize>,
    pub http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for ChannelDurationNotifier {
     async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let count_before = self.count.fetch_add(1, Ordering::Relaxed);
        self.chan_id
            .say(
                &self.http,
                &format!(
                    "I've been in this channel for {} minutes!",
                    count_before + 1
                ),
            )
            .await.expect("Error sending ping");

        None
    }
}