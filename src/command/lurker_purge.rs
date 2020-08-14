use crate::{command::CustomCommand, config::AppConfig, util};
use chrono::Utc;
use serenity::{
    framework::standard::CommandResult,
    model::{
        channel::{Message, ReactionType},
        guild::Member,
        id::{ChannelId, EmojiId, GuildId, RoleId},
        user::User,
    },
    prelude::Context,
};
use std::{sync::Arc, thread, time::Duration};

const SECONDS_IN_DAY: u64 = 86_400;

pub struct LurkerPurge<'a> {
    ctx: &'a Context,
    _msg: &'a Message,
    config: Arc<AppConfig>,
}

impl LurkerPurge<'_> {
    pub fn on_ready(config: Arc<AppConfig>, ctx: Context) {
        if let Some(ref purge_config) = config.lurker_purge {
            let channel_id = ChannelId(purge_config.channel_id);

            if let Some(Some(message)) = channel_id
                .messages(&ctx, |retriever| retriever.limit(1))
                .ok()
                .map(|x| x.into_iter().last())
            {
                // If last message was not a purge announcement, do not attempt to purge users
                let content = message
                    .embeds
                    .get(0)
                    .map(|x| x.description.clone().unwrap_or(String::new()));
                if content != Some(purge_config.message.clone()) {
                    return;
                }

                wait_for_grace_period_and_do_purge(config, ctx, message);
            }
        }
    }
}

impl<'a> CustomCommand<'a> for LurkerPurge<'a> {
    fn new(ctx: &'a Context, msg: &'a Message) -> Self {
        LurkerPurge {
            config: AppConfig::get_arc(),
            ctx,
            _msg: msg,
        }
    }

    fn exec(&self) -> CommandResult {
        if let Some(ref purge_config) = self.config.lurker_purge {
            let message = util::send_basic_embed(
                self.ctx,
                &ChannelId(purge_config.channel_id),
                &purge_config.message,
            )?;
            message.react(
                self.ctx,
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(731955992647958641),
                    name: Some("happybagelday".to_string()),
                },
            )?;

            wait_for_grace_period_and_do_purge(self.config.clone(), self.ctx.clone(), message);
        }

        Ok(())
    }
}

fn wait_for_grace_period_and_do_purge(config: Arc<AppConfig>, ctx: Context, message: Message) {
    if let Some(ref purge_config) = config.lurker_purge {
        let sleep_duration = {
            let elapsed_grace_period = Utc::now().timestamp() - message.timestamp.timestamp();
            let remaining_grace_period =
                (purge_config.grace_period_days * SECONDS_IN_DAY) - elapsed_grace_period as u64;

            Duration::from_secs(remaining_grace_period)
        };

        thread::spawn(move || {
            thread::sleep(sleep_duration);
            let reaction_users = ChannelId(config.lurker_purge.as_ref().unwrap().channel_id)
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

            let inactive_members = kick_inactive_members(config.clone(), &ctx, &reaction_users);

            announce_results_of_purge(config.clone(), &ctx, &reaction_users, &inactive_members);
        });
    }
}

fn inactive_members(
    config: Arc<AppConfig>,
    ctx: &Context,
    reaction_users: &Vec<User>,
) -> Vec<Member> {
    let purge_config = config.lurker_purge.as_ref().unwrap();
    GuildId(config.guild_id)
        .members_iter(&ctx)
        .map(|m| m.unwrap())
        .filter(|m| !reaction_users.iter().any(|r| r.id == m.user.read().id))
        .filter(|m| {
            !purge_config
                .immune_roles
                .iter()
                .map(|&x| RoleId(x))
                .any(|immune_role| {
                    m.roles
                        .iter()
                        .any(|&member_role| member_role == immune_role)
                })
        })
        .collect()
}

fn active_members(
    config: Arc<AppConfig>,
    ctx: &Context,
    reaction_users: &Vec<User>,
) -> Vec<Member> {
    let purge_config = config.lurker_purge.as_ref().unwrap();
    GuildId(config.guild_id)
        .members_iter(&ctx)
        .map(|m| m.unwrap())
        .filter(|m| reaction_users.iter().any(|r| r.id == m.user.read().id))
        .filter(|m| {
            !purge_config
                .immune_roles
                .iter()
                .map(|&x| RoleId(x))
                .any(|immune_role| {
                    m.roles
                        .iter()
                        .any(|&member_role| member_role == immune_role)
                })
        })
        .collect()
}

fn kick_inactive_members(
    config: Arc<AppConfig>,
    ctx: &Context,
    reaction_users: &Vec<User>,
) -> Vec<Member> {
    let inactive = inactive_members(config.clone(), &ctx, reaction_users);

    for user in inactive.iter() {
        user.kick_with_reason(ctx, "Kicked for inactivity").ok();
    }
    inactive
}

fn announce_results_of_purge(
    config: Arc<AppConfig>,
    ctx: &Context,
    reaction_users: &Vec<User>,
    inactive_members: &Vec<Member>,
) {
    let channel_id = ChannelId(config.lurker_purge.as_ref().unwrap().channel_id);
    let kicked = inactive_members
        .iter()
        .map(|m| format!("  - [x] ~~{}~~", m.user.read().name.clone()))
        .collect::<Vec<_>>()
        .join("\n");
    let survivors = active_members(config.clone(), &ctx, &reaction_users)
        .iter()
        .map(|m| format!("  - [·] **{}**", m.user.read().name.clone()))
        .collect::<Vec<_>>()
        .join("\n");

    let message = format!(
        "\
        **Thank you for your participation in the purge.**\n\
        \n\
        Remeber users who have fallen:\n{}\n\
        \n\
        Surviving users of the purge:\n{}\
    ",
        kicked, survivors
    );

    util::send_basic_embed(ctx, &channel_id, message).ok();
}
