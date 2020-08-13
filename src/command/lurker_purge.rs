use crate::{color, command::CustomCommand, config::AppConfig, util};
use chrono::Utc;
use serde_json::json;
use serenity::{
    framework::standard::CommandResult,
    model::{
        channel::{Message, ReactionType},
        guild::Member,
        id::{ChannelId, EmojiId, GuildId, RoleId},
    },
    prelude::Context,
};
use std::{sync::Arc, thread, time::Duration};

const SECONDS_IN_DAY: u64 = 86_400;

pub struct LurkerPurge<'a> {
    ctx: &'a Context,
    msg: &'a Message,
    config: Arc<AppConfig>,
}

impl LurkerPurge<'_> {
    pub fn on_ready(config: Arc<AppConfig>, ctx: Context) {
        // Handles lurker purge if enabled in config
        if let Some(ref purge_config) = config.lurker_purge {
            let channel_id = ChannelId(purge_config.channel_id);

            if let Some(Some(message)) = channel_id
                .messages(&ctx, |retriever| retriever.limit(1))
                .ok()
                .map(|x| x.into_iter().last())
            {
                let elapsed_grace_period = Utc::now().timestamp() - message.timestamp.timestamp();
                let remaining_grace_period =
                    (purge_config.grace_period_days * SECONDS_IN_DAY) - elapsed_grace_period as u64;
                let sleep_duration = Duration::from_secs(remaining_grace_period);

                let config = config.clone();
                thread::spawn(move || {
                    thread::sleep(sleep_duration);
                    // Get all memebers who reacted
                    let did_react = channel_id
                        .reaction_users(
                            &ctx,
                            message.id,
                            ReactionType::Custom {
                                animated: false,
                                id: EmojiId(731955992647958641),
                                name: Some("happybagelday".to_string()),
                            },
                            None,
                            None,
                        )
                        .unwrap();

                    // Get all members without immune roles who didn't react
                    if let Some(ref purge_config) = config.lurker_purge {
                        let inactive_users: Vec<Member> = GuildId(config.guild_id)
                            .members_iter(&ctx)
                            .map(|m| m.unwrap())
                            .filter(|m| !did_react.iter().any(|r| r.id == m.user.read().id))
                            .filter(|m| {
                                !purge_config
                                    .immune_roles
                                    .iter()
                                    .map(|&x| RoleId(x))
                                    .any(|ir| m.roles.iter().any(|&mr| mr == ir))
                            })
                            .collect();

                        // Kick the inactive users
                        for user in inactive_users.into_iter() {
                            user.kick_with_reason(&ctx, "Kicked for inactivity").ok();
                        }

                        // TODO: send message showing who was kicked and who survived
                    }
                });
            }
        }
    }
}

impl<'a> CustomCommand<'a> for LurkerPurge<'a> {
    fn new(ctx: &'a Context, msg: &'a Message) -> Self {
        LurkerPurge {
            config: AppConfig::get_arc(),
            ctx,
            msg,
        }
    }

    fn exec(&self) -> CommandResult {
        if let Some(ref purge_config) = self.config.lurker_purge {
            if !purge_config
                .authorized_user_ids
                .iter()
                .any(|id| id == &self.msg.author.id.0)
            {
                util::send_map(
                    self.ctx,
                    &self.msg.channel_id,
                    json!({
                        "tts": false,
                        "embed": {
                            "title": "[ ACCESS DENIED ]",
                            "color": color::RED
                        }
                    }),
                )?;
                return Ok(());
            }

            util::send_basic_embed(
                self.ctx,
                &ChannelId(purge_config.channel_id),
                &purge_config.message,
            )?
            .react(
                self.ctx,
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(731955992647958641),
                    name: Some("happybagelday".to_string()),
                },
            )?;
        } else {
            let map = json!({
                "tts": false,
                "embed": {
                    "description": "[ ERROR ]: No configuration for lurker_purge",
                    "color": color::LUMINOUS_VIVID_PINK
                }
            });
            util::send_map(self.ctx, &self.msg.channel_id, map)?;
        }

        Ok(())
    }
}
