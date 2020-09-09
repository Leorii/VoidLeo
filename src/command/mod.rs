use crate::AppConfig;
use serenity::{framework::standard::CommandResult, model::channel::Message, prelude::Context};

mod event;
mod lurker_purge;
mod ping;
mod purge_channel_messages;

pub use event::Event;
pub use lurker_purge::LurkerPurge;
pub use ping::Ping;
pub use purge_channel_messages::PurgeChannelMessages;

pub trait CustomCommand<'a> {
    fn new(ctx: &'a Context, msg: &'a Message) -> Self;
    fn exec(&self) -> CommandResult;

    fn authorized(msg: &Message) -> bool {
        let config = AppConfig::get_arc();

        match (
            config.command_permissions.ping.as_ref(),
            msg.member.as_ref(),
        ) {
            (Some(authorized_roles), Some(member)) => {
                member.roles.iter().fold(false, |acc, role| {
                    acc || authorized_roles.get(role).is_some()
                })
            }
            (None, _) => true,
            _ => false,
        }
    }
}
