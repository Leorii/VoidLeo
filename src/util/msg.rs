use crate::color;
use serenity::{
    model::{channel::Message, id::ChannelId},
    prelude::Context,
    utils::Color,
    Result,
};

pub struct Embed {
    ctx: Context,
    channel_id: ChannelId,
    title: Option<String>,
    description: Option<String>,
    color: u32,
}

impl Embed {
    pub fn new(ctx: &Context, channel_id: &ChannelId) -> Self {
        Embed {
            ctx: ctx.clone(),
            channel_id: channel_id.clone(),
            title: None,
            description: None,
            color: color::GOLD,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn descr(mut self, descr: &str) -> Self {
        self.description = Some(descr.into());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    pub fn send(self) -> Result<Message> {
        self.channel_id.send_message(self.ctx.clone(), move |m| {
            m.tts(true);
            m.embed(|e| {
                if let Some(title) = self.title {
                    e.title(title);
                }
                if let Some(description) = self.description {
                    e.description(description);
                }
                e.color(Color::new(self.color));
                e
            });
            m
        })
    }
}
