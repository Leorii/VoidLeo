use serenity::{framework::standard::CommandResult, model::channel::Message, prelude::Context};

mod lurker_purge;

pub use lurker_purge::LurkerPurge;

pub trait CustomCommand<'a> {
    fn new(ctx: &'a Context, msg: &'a Message) -> Self;
    fn exec(&self) -> CommandResult;
}
