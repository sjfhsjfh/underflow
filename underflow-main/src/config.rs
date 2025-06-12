use std::sync::OnceLock;

use tokio::sync::Mutex;

static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();
fn get_config() -> &'static Mutex<Config> {
    CONFIG.get_or_init(|| Mutex::new(Config::default()))
}

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub player_count: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self { player_count: 2 }
    }
}
