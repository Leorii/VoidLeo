use ron;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Default, Deserialize, Serialize)]
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

pub fn from_file<P: AsRef<Path>>(filename: P) -> AppConfig {
    let raw_config = fs::read_to_string(filename).expect("could not read config file");
    ron::from_str(&raw_config).expect("invalid config file format")
}
