use lazy_static::lazy_static;
use ron;
use serde::Deserialize;
use serenity::model::id::{ChannelId, GuildId, RoleId, UserId};
use std::{collections::HashSet, fs, sync::Arc};

lazy_static! {
    static ref CONFIG: Arc<AppConfig> = Arc::new(AppConfig::init());
}

#[derive(Clone, Default, Deserialize)]
pub struct AppConfig {
    pub discord_token: String,
    pub guild_id: GuildId,
    pub bot_user_id: UserId,
    pub owners: HashSet<UserId>,

    pub log_channel_id: Option<ChannelId>,
    pub new_member_welcome: Option<NewMemberWelcome>,
    pub emoji_pings: Option<Vec<EmojiPingConfig>>,
    pub lurker_purge: Option<LurkerPurgeConfig>,
}

#[derive(Clone, Default, Deserialize)]
pub struct CommandPermissions {
    pub event: Option<HashSet<RoleId>>,
    pub ping: Option<HashSet<RoleId>>,
}

#[derive(Clone, Deserialize)]
pub struct NewMemberWelcome {
    pub message: String,
    pub channel_id: ChannelId,
    pub ping_insert_idx: Option<usize>,
}

#[derive(Clone, Deserialize)]
pub struct EmojiPingConfig {
    pub user_id: UserId,
    pub emojis: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct LurkerPurgeConfig {
    pub channel_id: ChannelId,
    pub grace_period_days: u64,
    pub immune_roles: Vec<RoleId>,
    pub message: String,
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
