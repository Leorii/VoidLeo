use serenity::{framework::standard::CommandResult, model::channel::Message, prelude::Context};

mod event;
mod lurker_purge;

pub use event::Event;
pub use lurker_purge::LurkerPurge;

pub trait CustomCommand<'a> {
    fn new(ctx: &'a Context, msg: &'a Message) -> Self;
    fn exec(&self) -> CommandResult;
}
