use crate::{
    color,
    command::CustomCommand,
    util::{self, Embed, Logger},
    AppConfig,
};
use serenity::{
    framework::standard::{CommandError, CommandResult},
    model::{
        channel::Message,
        id::{ChannelId, MessageId},
    },
    prelude::Context,
};
use std::sync::Arc;

pub struct PurgeChannelMessages<'a> {
    ctx: &'a Context,
    msg: &'a Message,
    _config: Arc<AppConfig>,
}

impl<'a> CustomCommand<'a> for PurgeChannelMessages<'a> {
    fn new(ctx: &'a Context, msg: &'a Message) -> Self {
        PurgeChannelMessages {
            ctx,
            msg,
            _config: AppConfig::get_arc(),
        }
    }

    fn exec(&self) -> CommandResult {
        let channel_id = self.msg.channel_id;
        let logger = Logger::new(self.ctx);
        let args = self.parse_args().map_err(|e| {
            logger.error(&format!(
                "Could not parse arg string for purge_channel_messages: {:?}",
                e
            ));
            e
        })?;

        if let Some((confirmation_id, after)) = args {
            if confirmation_id == channel_id {
                // Discord limits us to getting 100 messages at a time, so we need to loop to get
                // them all.
                loop {
                    let messages = channel_id
                        .messages(self.ctx, |mut retriever| {
                            if let Some(after_id) = after {
                                retriever = retriever.after(after_id);
                            }
                            retriever.limit(100)
                        })
                        .map_err(|e| {
                            logger.error(&format!(
                                "Could not retrieve messages for purge_channel_messages: {}",
                                e
                            ));
                            e
                        })?;

                    if messages.len() < 1 {
                        return Ok(());
                    }

                    for message in messages.iter() {
                        message.delete(self.ctx)?;
                    }
                }
            } else {
                Embed::new(self.ctx, &channel_id)
                    .descr("The confirmation id does not match this channel id!")
                    .color(color::RED)
                    .send()
                    .map_err(|e| {
                        logger.error(&format!(
                            "Could not send purge_channel_messages id warning: {}",
                            e
                        ));
                        e
                    })?;
            }
        } else {
            Embed::new(self.ctx, &channel_id)
                .descr("\
                   This will delete **ALL** messages in this channel, since the beginning of time. Are you \
                   **ABSOLUTELY SURE** that this is what you want to do? \
                   \
                   To confirm, use the command again with the following option: `--confirm=<channel_id>`\
                ")
                .color(color::LUMINOUS_VIVID_PINK)
                .send()
                .map_err(|e| {
                    logger.error(&format!("Could not send purge_channel_messages confirmation: {}", e));
                    e
                })?;
        }

        Ok(())
    }
}

impl PurgeChannelMessages<'_> {
    fn parse_args(&self) -> Result<Option<(ChannelId, Option<MessageId>)>, CommandError> {
        let arg_string = match util::get_arg_string(self.msg, "purge_channel_messages") {
            Some(x) => x,
            None => return Ok(None),
        };
        let mut confirmation: Option<ChannelId> = None;
        let mut after: Option<MessageId> = None;

        for arg in arg_string.split_whitespace() {
            if arg.starts_with("--confirm=") {
                let id = arg.chars().skip(10).collect::<String>();
                confirmation = Some(ChannelId(id.parse()?));
                continue;
            }
            if arg.starts_with("--after=") {
                let id = arg.chars().skip(8).collect::<String>();
                after = Some(MessageId(id.parse()?));
                continue;
            }
        }

        if let Some(confirmation) = confirmation {
            return Ok(Some((confirmation, after)));
        }

        Ok(None)
    }
}
