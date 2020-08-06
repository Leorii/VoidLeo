use super::color;
use serde_json::{json, Value};
use serenity::{
    http::CacheHttp,
    model::{channel::Message, id::ChannelId},
    Result,
};

pub struct Embed {
    pub title: Option<String>,
    pub descr: Option<String>,
    pub color: Option<usize>,
}

pub fn send_msg(
    ctx: impl CacheHttp,
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
    ctx: impl CacheHttp,
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

pub fn send_map(ctx: impl CacheHttp, channel_id: &ChannelId, map: Value) -> Result<Message> {
    ctx.http().send_message(channel_id.0, &map)
}
