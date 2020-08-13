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
#[commands(ping, lurker_purge)]
struct General;

fn main() {
    let config = AppConfig::get_arc();
    let mut client = Client::new(&config.discord_token, Handler::new(config.clone()))
        .expect("Error creating client");
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
    command::LurkerPurge::new(ctx, msg).exec()
}
