use crate::color;
use serde_json::{json, Value};
use serenity::{
    http::CacheHttp,
    model::{channel::Message, id::ChannelId},
    prelude::Context,
    Result,
};

pub fn send_text(
    ctx: &Context,
    channel_id: &ChannelId,
    content: impl AsRef<str>,
) -> Result<Message> {
    let map = json!({
        "content": content.as_ref(),
        "tts": false,
    });
    send_map(ctx, channel_id, map)
}

pub fn send_basic_embed(
    ctx: &Context,
    channel_id: &ChannelId,
    content: impl AsRef<str>,
) -> Result<Message> {
    let map = json!({
        "tts": false,
        "embed": {
            "description": content.as_ref(),
            "color": color::GOLD
        }
    });
    send_map(ctx, channel_id, map)
}

pub fn send_map(ctx: &Context, channel_id: &ChannelId, map: Value) -> Result<Message> {
    ctx.http().send_message(channel_id.0, &map)
}
