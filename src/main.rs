#[macro_use]
extern crate lazy_static;

use serde_json::json;
use serenity::{
    client::Client,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{
        channel::{Message, ReactionType},
        id::{ChannelId, EmojiId},
    },
    prelude::{Context, EventHandler},
};
use std::sync::RwLock;
use voidleo::{color, config::AppConfig, util};

lazy_static! {
    static ref CONFIG: RwLock<AppConfig> = RwLock::new(AppConfig::default());
}

#[group]
#[commands(ping, lurker_purge)]
struct General;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let config = CONFIG.read().unwrap();

        // Handles emoji pings if enabled in config
        if let Some(ref emoji_pings) = config.emoji_pings {
            for user_id in emoji_pings
                .iter()
                .filter(|ep| ep.emojis.iter().any(|e| e == &msg.content))
                .map(|ep| &ep.user_id)
            {
                if let Some(ping) =
                    util::send_msg(&ctx, &msg.channel_id, format!("<@{}>", user_id)).ok()
                {
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
