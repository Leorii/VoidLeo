use serde_json::{json, Value};
use serenity::{
    http::CacheHttp,
    model::{channel::Message, id::ChannelId},
    Result,
};

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
pub fn send_map(ctx: impl CacheHttp, channel_id: &ChannelId, map: Value) -> Result<Message> {
    ctx.http().send_message(channel_id.0, &map)
}
