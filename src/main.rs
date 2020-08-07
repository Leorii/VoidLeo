#[macro_use]
extern crate lazy_static;

use serde_json::json;
use serenity::{
    client::Client,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::channel::Message,
    prelude::{Context, EventHandler},
};
use std::sync::RwLock;
use voidleo::{
    color,
    config::{self, AppConfig},
    util,
};

lazy_static! {
    static ref CONFIG: RwLock<AppConfig> = RwLock::new(AppConfig::default());
}

#[group]
#[commands(ping, lurker_prune)]
struct General;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    let config = {
        let mut config = CONFIG.write().unwrap();
        *config = config::from_file("./config.ron");
        CONFIG.read().unwrap()
    };
    fn create_client(token: &str) -> Client {
        let mut client = Client::new(token, Handler).expect("Error creating client");

        client.with_framework(
            StandardFramework::new()
                .configure(|c| c.prefix("::"))
                .group(&GENERAL_GROUP),
        );
        client
    }

    fn start_client(mut client: Client) {
        if let Err(e) = client.start() {
            println!("An error occurred while running the client: {:?}", e);
        }
    }

    start_client(create_client(&config.discord_token));
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn lurker_prune(ctx: &mut Context, msg: &Message) -> CommandResult {
    let config = CONFIG.read().unwrap();

    if msg.author.id.0.to_string() != config.guild_owner_id {
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
        ctx,
        &msg.channel_id,
        "Yo!\n\nThis is a test. Don't mind me. <:happybagelday:731955992647958641>",
    )?;
    Ok(())
}
