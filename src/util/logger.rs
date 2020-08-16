use crate::{color, util::msg};
use log::{error, info, warn};
use serenity::{model::id::ChannelId, prelude::Context};

pub enum Logger {
    StdLogOnly,
    WithLogChannel { ctx: Context, channel_id: ChannelId },
}

impl Logger {
    pub fn new(with_log_channel: Option<(Context, ChannelId)>) -> Self {
        if let Some((ctx, channel_id)) = with_log_channel {
            Logger::WithLogChannel { ctx, channel_id }
        } else {
            Logger::StdLogOnly
        }
    }

    pub fn error(&self, message: &str) {
        if let Logger::WithLogChannel { ctx, channel_id } = self {
            if let Err(e) = msg::Embed::new(&ctx, &channel_id)
                .title("[ ERROR ]")
                .descr(message)
                .color(color::LUMINOUS_VIVID_PINK)
                .send()
            {
                warn!("Could not send error message to log channel: {}", e);
            }
        }

        error!("{}", message);
    }

    pub fn warn(&self, message: &str) {
        if let Logger::WithLogChannel { ctx, channel_id } = self {
            if let Err(e) = msg::Embed::new(&ctx, &channel_id)
                .title("[ WARN ]")
                .descr(message)
                .color(color::RED)
                .send()
            {
                warn!("Could not send warn message to log channel: {}", e);
            }
        }

        warn!("{}", message);
    }

    pub fn info(&self, message: &str) {
        if let Logger::WithLogChannel { ctx, channel_id } = self {
            if let Err(e) = msg::Embed::new(&ctx, &channel_id)
                .title("[ INFO ]")
                .descr(message)
                .send()
            {
                warn!("Could not send info message to log channel: {}", e);
            }
        }

        info!("{}", message);
    }
}
