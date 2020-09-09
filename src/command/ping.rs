use crate::command::CustomCommand;
use serenity::{framework::standard::CommandResult, model::channel::Message, prelude::Context};

pub struct Ping<'a> {
    ctx: &'a Context,
    msg: &'a Message,
}

impl<'a> CustomCommand<'a> for Ping<'a> {
    fn new(ctx: &'a Context, msg: &'a Message) -> Self {
        Ping { ctx, msg }
    }

    fn exec(&self) -> CommandResult {
        if !Ping::authorized(self.msg) {
            return Ok(());
        }
        self.msg.reply(self.ctx, "Pong!")?;

        Ok(())
    }
}
