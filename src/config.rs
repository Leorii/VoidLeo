use ron;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub discord_token: String,
    pub guild_id: u64,
    pub emoji_pings: Option<Vec<EmojiPingConfig>>,
    pub lurker_purge: Option<LurkerPurgeConfig>,
}

#[derive(Deserialize, Serialize)]
pub struct EmojiPingConfig {
    pub user_id: u64,
    pub emojis: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LurkerPurgeConfig {
    pub channel_id: u64,
    pub grace_period_days: u64,
    pub immune_roles: Vec<u64>,
    pub message: String,
    pub authorized_user_ids: Vec<u64>,
}

impl AppConfig {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        let raw_config = fs::read_to_string(filename).expect("could not read config file");
        ron::from_str(&raw_config).expect("invalid config file format")
    }
}
