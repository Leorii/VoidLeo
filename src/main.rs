use serenity::{
    client::Client,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::channel::Message,
    prelude::{Context, EventHandler},
};
use std::env;

#[group]
#[commands(Ping)]
struct General;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token missing");
    fn create_client(token: &str) -> Client {
        let mut client = Client::new(token, Handler).expect("Error creating client");

        client.with_framework(
            StandardFramework::new()
                .configure(|c| c.prefix("0x"))
                .group(&GENERAL_GROUP),
        );
        client
    }

    fn start_client(mut client: Client) {
        if let Err(e) = client.start() {
            println!("An error occurred while running the client: {:?}", e);
        }
    }

    start_client(create_client(&token));
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
