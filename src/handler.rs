use crate::{command, AppConfig};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::{Context, EventHandler},
};
use std::sync::Arc;

pub struct Handler {
    config: Arc<AppConfig>,
}

impl Handler {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Handler { config }
    }
}

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        command::LurkerPurge::on_ready(self.config.clone(), ctx);
    }

    fn message(&self, ctx: Context, msg: Message) {
        // Handles emoji pings if enabled in config
        if let Some(ref emoji_pings) = self.config.emoji_pings {
            for user_id in emoji_pings
                .iter()
                .filter(|ep| ep.emojis.iter().any(|e| e == &msg.content))
                .map(|ep| &ep.user_id)
            {
                if let Some(ping) = msg.channel_id.say(&ctx, format!("<@{}>", user_id)).ok() {
                    ping.delete(&ctx).ok();
                }
            }
        }
    }
}
