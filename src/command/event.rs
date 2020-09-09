use crate::{
    command::CustomCommand,
    util::{self, Embed, Logger},
    AppConfig,
};
use serenity::{
    framework::standard::{CommandError, CommandResult},
    model::{
        channel::{Message, ReactionType},
        id::{EmojiId, UserId},
    },
    prelude::Context,
};
use std::{sync::Arc, thread};

pub struct Event<'a> {
    ctx: &'a Context,
    msg: &'a Message,
    config: Arc<AppConfig>,
}

impl<'a> CustomCommand<'a> for Event<'a> {
    fn new(ctx: &'a Context, msg: &'a Message) -> Self {
        Event {
            ctx,
            msg,
            config: AppConfig::get_arc(),
        }
    }

    fn exec(&self) -> CommandResult {
        let channel_id = self.msg.channel_id;
        let logger = Logger::new(self.ctx);
        let (role, emoji, message) = match self.parse_args()? {
            Some(x) => x,
            None => return Ok(()),
        };

        let message = Embed::new(self.ctx, &channel_id)
            .content(&role)
            .descr(&format!(
                "{}\n\nReact with {} if you can make it!",
                message, emoji
            ))
            .send()
            .map_err(|e| {
                logger.error(&format!("Could not send event message: {}", &e));
                e
            })?;

        let emoji_id = {
            let mut id = emoji.split(':').nth(2).unwrap().to_string();
            id.pop();
            EmojiId(id.parse()?)
        };
        let reaction_type = ReactionType::Custom {
            animated: false,
            id: emoji_id,
            name: Some(emoji.split(':').nth(1).unwrap().to_string()),
        };

        message
            .react(self.ctx, reaction_type.clone())
            .map_err(|e| {
                logger.warn(&format!("Could not add reaction to event message: {}", &e));
                e
            })?;

        self.msg.delete(self.ctx)?;

        thread::spawn({
            let ctx = self.ctx.clone();
            let user_id = UserId(self.config.bot_user_id);

            move || {
                thread::sleep(std::time::Duration::from_secs(60));

                match channel_id.delete_reaction(ctx, message.id, Some(user_id), reaction_type) {
                    Err(e) => logger.error(&format!(
                        "Could not remove own reaction from event message: {}",
                        &e
                    )),
                    _ => {}
                }
            }
        });

        Ok(())
    }
}

impl Event<'_> {
    fn parse_args(&self) -> Result<Option<(String, String, String)>, CommandError> {
        let usage = "Usage: `event [role] <reaction_emoji> <message>`";
        let channel_id = self.msg.channel_id;

        let arg_string = match util::get_arg_string(self.msg, "event") {
            Some(x) => x,
            None => return Ok(None),
        };
        let mut arg_iter = arg_string.split(' ');

        let mut role: String = "@everyone".to_string();
        let mut emoji: String = "".to_string();
        match arg_iter.next() {
            Some(role_or_emoji) => {
                if &role_or_emoji.chars().take(2).collect::<String>() == "<@" {
                    role = role_or_emoji.to_string();
                } else {
                    emoji = role_or_emoji.to_string();
                }
            }
            None => {
                Embed::new(self.ctx, &channel_id)
                    .descr(&format!("No arguments supplied! {}", usage))
                    .send()
                    .map_err(|e| format!("Could not send usage message for event: {}", e))?;
                return Ok(None);
            }
        }

        if &emoji == "" {
            match arg_iter.next() {
                Some(x) => {
                    emoji = x.to_string();
                }
                None => {
                    Embed::new(self.ctx, &channel_id)
                        .descr(&format!("No reaction_emoji supplied!\n{}", usage))
                        .send()
                        .map_err(|e| format!("Could not send usage message for event: {}", e))?;
                    return Ok(None);
                }
            }
        }

        if &emoji[0..2] != "<:" {
            Embed::new(self.ctx, &channel_id)
                .descr(&format!(
                    "{} is not a valid reaction_emoji!\n{}",
                    emoji, usage
                ))
                .send()
                .map_err(|e| format!("Could not send usage message for event: {}", e))?;
            return Ok(None);
        }

        let message = arg_iter.collect::<Vec<&str>>().join(" ");

        if &message == "" {
            Embed::new(self.ctx, &channel_id)
                .descr(&format!("no message was supplied!\n{}", usage))
                .send()
                .map_err(|e| format!("Could not send usage message for event: {}", e))?;
            return Ok(None);
        }

        Ok(Some((role, emoji, message)))
    }
}
