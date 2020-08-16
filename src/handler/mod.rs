use crate::{command, util::Logger, AppConfig};
use serenity::{
    model::{
        channel::Message,
        gateway::Ready,
        guild::Member,
        id::{ChannelId, GuildId},
    },
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
        let logger = Logger::new(
            self.config
                .log_channel_id
                .map(|id| (ctx.clone(), ChannelId(id))),
        );

        // Handles emoji pings if enabled in config
        if let Some(ref emoji_pings) = self.config.emoji_pings {
            for user_id in emoji_pings
                .iter()
                .filter(|ep| ep.emojis.iter().any(|e| e == &msg.content))
                .map(|ep| &ep.user_id)
            {
                if let Some(ping) = msg
                    .channel_id
                    .say(&ctx, format!("<@{}>", user_id))
                    .map_err(|e| {
                        logger.error(&format!(
                            "Unable to send emoji ping for <@{}>: {}",
                            user_id, e
                        ))
                    })
                    .ok()
                {
                    ping.delete(&ctx)
                        .map_err(|e| {
                            logger.warn(&format!(
                                "Unable to delete emoji ping message for <@{}>: {}",
                                user_id, e,
                            ))
                        })
                        .ok();
                }
            }
        }
    }

    fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, new_member: Member) {
        let logger = Logger::new(
            self.config
                .log_channel_id
                .map(|id| (ctx.clone(), ChannelId(id))),
        );

        // Handles new member welcome messages if enabled in config
        if let Some(welcome_config) = &self.config.new_member_welcome {
            let channel_id = ChannelId(welcome_config.channel_id);
            let user_ping = format!("<@{}>", new_member.user.read().id);
            let message = if let Some(i) = welcome_config.ping_insert_idx {
                let mut m = welcome_config.message.clone();
                m.insert_str(i, &user_ping);
                m
            } else {
                welcome_config.message.clone()
            };

            channel_id
                .say(ctx, message)
                .map_err(|e| {
                    logger.error(&format!(
                        "Unable to send welcome message to {}: {}",
                        user_ping, e
                    ))
                })
                .ok();
        }
    }
}
