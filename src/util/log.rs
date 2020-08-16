use crate::{color, util::msg};
use log::{error, info, warn};
use serde_json::json;
use serenity::{model::id::ChannelId, prelude::Context};

pub enum Logger {
    StdLogOnly,
    WithLogChannel { ctx: Context, channel_id: ChannelId },
}

impl Logger {
    pub fn new() -> Self {
        Logger::StdLogOnly
    }

    pub fn with_log_channel(ctx: Context, channel_id: ChannelId) -> Self {
        Logger::WithLogChannel { ctx, channel_id }
    }

    pub fn error(&self, message: &str) {
        if let Logger::WithLogChannel { ctx, channel_id } = self {
            let content = json!({
                "tts": false,
                "embed": {
                    "title": "[ ERROR ]",
                    "description": message,
                    "color": color::RED,
                }
            });
            if let Err(e) = msg::send_map(&ctx, &channel_id, content) {
                warn!("Could not send error message to log channel: {}", e);
            }
        }

        error!("{}", message);
    }

    pub fn warn(&self, message: &str) {
        if let Logger::WithLogChannel { ctx, channel_id } = self {
            let content = json!({
                "tts": false,
                "embed": {
                    "title": "[ WARN ]",
                    "description": message,
                    "color": color::LUMINOUS_VIVID_PINK,
                }
            });
            if let Err(e) = msg::send_map(&ctx, &channel_id, content) {
                warn!("Could not send warn message to log channel: {}", e);
            }
        }

        warn!("{}", message);
    }

    pub fn info(&self, message: &str) {
        if let Logger::WithLogChannel { ctx, channel_id } = self {
            let content = json!({
                "tts": false,
                "embed": {
                    "title": "[ INFO ]",
                    "description": message,
                    "color": color::GOLD,
                }
            });
            if let Err(e) = msg::send_map(&ctx, &channel_id, content) {
                warn!("Could not send info message to log channel: {}", e);
            }
        }

        info!("{}", message);
    }
}
