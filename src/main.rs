#[macro_use]
extern crate lazy_static;

use chrono::offset::Utc;
use serde_json::json;
use serenity::{
    client::Client,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{
        channel::{Message, ReactionType},
        gateway::Ready,
        guild::Member,
        id::{ChannelId, EmojiId, GuildId, RoleId},
    },
    prelude::{Context, EventHandler},
};
use std::{sync::RwLock, thread, time::Duration};
use voidleo::{color, config::AppConfig, util};

lazy_static! {
    static ref CONFIG: RwLock<AppConfig> = RwLock::new(AppConfig::default());
}

const SECONDS_IN_DAY: u64 = 86_400;

#[group]
#[commands(ping, lurker_purge)]
struct General;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        let config = CONFIG.read().unwrap();

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
                    let config = CONFIG.read().unwrap();
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
                    }
                });
            }
        }
    }

    fn message(&self, ctx: Context, msg: Message) {
        let config = CONFIG.read().unwrap();

        // Handles emoji pings if enabled in config
        if let Some(ref emoji_pings) = config.emoji_pings {
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

fn main() {
    {
        let mut config = CONFIG.write().unwrap();
        *config = AppConfig::from_file("./config.ron");
    }
    let config = CONFIG.read().unwrap();

    let mut client = Client::new(&config.discord_token, Handler).expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("::"))
            .group(&GENERAL_GROUP),
    );

    if let Err(e) = client.start() {
        println!("An error occurred while running the client: {:?}", e);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn lurker_purge(ctx: &mut Context, msg: &Message) -> CommandResult {
    let config = CONFIG.read().unwrap();

    if let Some(ref purge_config) = config.lurker_purge {
        if !purge_config
            .authorized_user_ids
            .iter()
            .any(|id| id == &msg.author.id.0)
        {
            util::send_map(
                ctx,
                &msg.channel_id,
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
            &ctx,
            &ChannelId(purge_config.channel_id),
            &purge_config.message,
        )?
        .react(
            &ctx,
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
        util::send_map(&ctx, &msg.channel_id, map)?;
    }

    Ok(())
}
