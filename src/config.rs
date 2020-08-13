use lazy_static::lazy_static;
use ron;
use serde::Deserialize;
use std::{fs, sync::Arc};

lazy_static! {
    static ref CONFIG: Arc<AppConfig> = Arc::new(AppConfig::init());
}

#[derive(Clone, Default, Deserialize)]
pub struct AppConfig {
    pub discord_token: String,
    pub guild_id: u64,
    pub emoji_pings: Option<Vec<EmojiPingConfig>>,
    pub lurker_purge: Option<LurkerPurgeConfig>,
}

#[derive(Clone, Deserialize)]
pub struct EmojiPingConfig {
    pub user_id: u64,
    pub emojis: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct LurkerPurgeConfig {
    pub channel_id: u64,
    pub grace_period_days: u64,
    pub immune_roles: Vec<u64>,
    pub message: String,
    pub authorized_user_ids: Vec<u64>,
}

impl<'a> AppConfig {
    pub fn get_arc() -> Arc<Self> {
        CONFIG.clone()
    }

    fn init() -> Self {
        let raw_config = fs::read_to_string("./config.ron").expect("could not read config file");
        ron::from_str(&raw_config).expect("invalid config file format")
    }
}
