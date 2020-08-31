use serenity::{framework::standard::CommandResult, model::channel::Message, prelude::Context};

mod event;
mod lurker_purge;
mod purge_channel_messages;

pub use event::Event;
pub use lurker_purge::LurkerPurge;
pub use purge_channel_messages::PurgeChannelMessages;

pub trait CustomCommand<'a> {
    fn new(ctx: &'a Context, msg: &'a Message) -> Self;
    fn exec(&self) -> CommandResult;
}
