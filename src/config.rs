use ron;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub discord_token: String,
    pub guild_owner_id: String,
    pub bot_id: String,
    pub emoji_pings: Option<Vec<EmojiPingConfig>>,
    pub lurker_purge: Option<LurkerPurgeConfig>,
}

#[derive(Deserialize, Serialize)]
pub struct EmojiPingConfig {
    pub user_id: String,
    pub emoji: String,
}

#[derive(Deserialize, Serialize)]
pub struct LurkerPurgeConfig {
    pub channel_id: usize,
    pub grace_period_days: u16,
    pub immune_roles: Vec<String>,
}

impl AppConfig {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        let raw_config = fs::read_to_string(filename).expect("could not read config file");
        ron::from_str(&raw_config).expect("invalid config file format")
    }
}
