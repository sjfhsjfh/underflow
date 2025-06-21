use underflow_l10n::{FALLBACK_LANG, GLOBAL, LANGS, set_prefered_locale};

pub static mut CONFIG: Option<Config> = None;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub player_count: u8,
    pub language: Option<String>,
}

pub fn set_config(data: Config) {
    unsafe {
        CONFIG = Some(data);
    }
}

#[allow(static_mut_refs)]
pub fn get_config() -> &'static Config {
    unsafe { CONFIG.as_ref().unwrap() }
}

#[allow(static_mut_refs)]
pub fn get_config_mut() -> &'static mut Config {
    unsafe { CONFIG.as_mut().unwrap() }
}

pub fn sync_config() {
    set_prefered_locale(
        get_config()
            .language
            .as_ref()
            .and_then(|it| it.parse().ok()),
    );
    if get_config().language.is_none() {
        get_config_mut().language = Some(LANGS[GLOBAL.order.lock().unwrap()[0]].to_owned());
    }
}

// TODO: save config

impl Default for Config {
    fn default() -> Self {
        Self {
            player_count: 2,
            language: Some(FALLBACK_LANG.to_string()),
        }
    }
}
