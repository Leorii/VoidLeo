use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    pub discord_token: String,
    pub guild_owner_id: String,
    pub bot_id: String,
    pub emoji_pings: Option<Vec<EmojiPingConfig>>,
}

#[derive(Deserialize, Serialize)]
pub struct EmojiPingConfig {
    pub user_id: String,
    pub emoji: String,
}
