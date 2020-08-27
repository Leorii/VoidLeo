extern crate env_logger;

use serenity::{
    client::Client,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::channel::Message,
    prelude::Context,
};
use voidleo::{
    command::{self, CustomCommand},
    AppConfig, Handler,
};

#[group]
#[commands(event, lurker_purge, ping)]
struct General;

fn main() {
    env_logger::init();

    let config = AppConfig::get_arc();
    let mut client = Client::new(&config.discord_token, Handler::new(config.clone()))
        .expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.prefix("$")
                    .with_whitespace(true)
                    .owners(config.owners.clone())
            })
            .group(&GENERAL_GROUP),
    );

    if let Err(e) = client.start() {
        println!("An error occurred while running the client: {:?}", e);
    }
}

#[command]
fn event(ctx: &mut Context, msg: &Message) -> CommandResult {
    command::Event::new(ctx, msg).exec()
}

#[command]
#[owners_only]
fn lurker_purge(ctx: &mut Context, msg: &Message) -> CommandResult {
    command::LurkerPurge::new(ctx, msg).exec()
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
