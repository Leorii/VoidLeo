use crate::{
    command::CustomCommand,
    util::{Embed, Logger},
    AppConfig,
};
use serenity::{
    framework::standard::{CommandError, CommandResult},
    model::{
        channel::{Message, ReactionType},
        id::{ChannelId, EmojiId},
    },
    prelude::Context,
};
use std::sync::Arc;

pub struct PurgeChannelMessages<'a> {
    ctx: &'a Context,
    msg: &'a Message,
    config: Arc<AppConfig>,
}

impl<'a> CustomCommand<'a> for PurgeChannelMessages<'a> {
    fn new(ctx: &'a Context, msg: &'a Message) -> Self {
        PurgeChannelMessages {
            ctx,
            msg,
            config: AppConfig::get_arc(),
        }
    }

    fn exec(&self) -> CommandResult {
        Ok(())
    }
}
