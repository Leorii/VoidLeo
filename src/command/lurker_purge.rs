use crate::{
    command::CustomCommand,
    config::AppConfig,
    util::{Embed, Logger},
};
use chrono::Utc;
use lazy_static::lazy_static;
use serenity::{
    framework::standard::CommandResult,
    model::{
        channel::{Message, ReactionType},
        guild::Member,
        id::EmojiId,
        user::User,
    },
    prelude::Context,
};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const SECONDS_IN_DAY: u64 = 86_400;

lazy_static! {
    static ref THREAD_LOCK: Mutex<()> = Mutex::new(());
}

pub struct LurkerPurge<'a> {
    ctx: &'a Context,
    _msg: &'a Message,
    config: Arc<AppConfig>,
}

impl LurkerPurge<'_> {
    pub fn on_ready(config: Arc<AppConfig>, ctx: Context) {
        let purge_config = match config.lurker_purge.as_ref() {
            Some(x) => x,
            None => return (),
        };
        let logger = Logger::new(&ctx);

        // Only attempt to purge users if last message was a purge announcement
        if let Some(Some(message)) = purge_config
            .channel_id
            .messages(&ctx, |retriever| retriever.limit(1))
            .map_err(|e| {
                logger.error(&format!(
                    "Unable to retrieve messages for LurkerPurge on_ready: {}",
                    e
                ))
            })
            .ok()
            .map(|x| x.into_iter().last())
        {
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

impl<'a> CustomCommand<'a> for LurkerPurge<'a> {
    fn new(ctx: &'a Context, msg: &'a Message) -> Self {
        LurkerPurge {
            config: AppConfig::get_arc(),
            ctx,
            _msg: msg,
        }
    }

    fn exec(&self) -> CommandResult {
        let logger = Logger::new(self.ctx);
        let purge_config = match self.config.lurker_purge.as_ref() {
            Some(x) => x,
            None => return Ok(()),
        };
        let message = Embed::new(&self.ctx, &purge_config.channel_id)
            .content("@everyone")
            .descr(&purge_config.message)
            .send()
            .map_err(|e| {
                logger.error(&format!("Could not send lurker_purge message: {}", &e));
                e
            })?;

        message
            .react(
                self.ctx,
                ReactionType::Custom {
                    animated: false,
                    id: EmojiId(731955992647958641),
                    name: Some("happybagelday".to_string()),
                },
            )
            .map_err(|e| {
                logger.warn(&format!(
                    "Could not add reaction to lurker_purge message: {}",
                    &e
                ));
                e
            })?;

        wait_for_grace_period_and_do_purge(self.config.clone(), self.ctx.clone(), message);

        Ok(())
    }
}

fn wait_for_grace_period_and_do_purge(config: Arc<AppConfig>, ctx: Context, message: Message) {
    // we should only have one of these threads running at a time
    match THREAD_LOCK.try_lock() {
        Ok(_) => (),
        Err(_) => return,
    };

    let logger = Logger::new(&ctx);
    let purge_config = match config.lurker_purge.as_ref() {
        Some(x) => x,
        None => return (),
    };
    let sleep_duration = {
        let elapsed_grace_period = Utc::now().timestamp() - message.timestamp.timestamp();
        let remaining_grace_period =
            (purge_config.grace_period_days * SECONDS_IN_DAY) - elapsed_grace_period as u64;

        Duration::from_secs(remaining_grace_period)
    };

    thread::spawn(move || {
        thread::sleep(sleep_duration);
        let reaction_users = config
            .lurker_purge
            .as_ref()
            .unwrap()
            .channel_id
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
    })
    .join()
    .map_err(|e| logger.error(&format!("lurker_purge thread panic! {:?}", e)))
    .ok();
}

fn kick_inactive_members(
    config: Arc<AppConfig>,
    ctx: &Context,
    reaction_users: &Vec<User>,
) -> Vec<Member> {
    let logger = Logger::new(ctx);
    let inactive = inactive_members(config.clone(), ctx, reaction_users);

    for member in inactive.iter() {
        let user = member.user.read();
        match member.kick_with_reason(ctx, "Kicked for inactivity") {
            Ok(_) => logger.info(&format!(
                "Kicked **{}** for inactivity as a result of the purge",
                user.name
            )),
            Err(e) => logger.error(&format!(
                "Unable to kick <@{}> during purge: {}",
                user.id, e
            )),
        }
    }
    inactive
}

fn inactive_members(
    config: Arc<AppConfig>,
    ctx: &Context,
    reaction_users: &Vec<User>,
) -> Vec<Member> {
    let purge_config = config.lurker_purge.as_ref().unwrap();
    config
        .guild_id
        .members_iter(&ctx)
        .map(|m| m.unwrap())
        .filter(|m| !reaction_users.iter().any(|r| r.id == m.user.read().id))
        .filter(|m| {
            !purge_config
                .immune_roles
                .iter()
                .any(|immune_role| m.roles.iter().any(|member_role| member_role == immune_role))
        })
        .collect()
}

fn announce_results_of_purge(
    config: Arc<AppConfig>,
    ctx: &Context,
    reaction_users: &Vec<User>,
    inactive_members: &Vec<Member>,
) {
    let logger = Logger::new(ctx);
    let channel_id = config.lurker_purge.as_ref().unwrap().channel_id;
    let kicked = inactive_members
        .iter()
        .map(|m| format!("  - [x] ~~{}~~", m.user.read().name.clone()))
        .collect::<Vec<_>>()
        .join("\n");
    let survivors = active_members(config.clone(), ctx, reaction_users)
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

    Embed::new(ctx, &channel_id)
        .descr(&message)
        .send()
        .err()
        .map(|e| logger.warn(&format!("Could not announce results of purge: {}", e)));
}

fn active_members(
    config: Arc<AppConfig>,
    ctx: &Context,
    reaction_users: &Vec<User>,
) -> Vec<Member> {
    let purge_config = config.lurker_purge.as_ref().unwrap();
    config
        .guild_id
        .members_iter(&ctx)
        .map(|m| m.unwrap())
        .filter(|m| reaction_users.iter().any(|r| r.id == m.user.read().id))
        .filter(|m| {
            !purge_config
                .immune_roles
                .iter()
                .any(|immune_role| m.roles.iter().any(|member_role| member_role == immune_role))
        })
        .collect()
}
