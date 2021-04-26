const DEFAULT_CONFIG_YAML: &str = include_str!("config.yml");

use crate::config::{self, Config};
use lazy_static::lazy_static;

lazy_static! {
    static ref DEFAULT_CONFIG: Config = serde_yaml::from_str(DEFAULT_CONFIG_YAML).unwrap();
}

pub fn version() -> String {
    DEFAULT_CONFIG.version().clone()
}

pub fn general() -> config::GeneralConfig {
    DEFAULT_CONFIG.general().clone()
}

pub fn node_types() -> config::NodeTypesConfig {
    DEFAULT_CONFIG.node_types().clone()
}

pub fn modes() -> config::ModesConfig {
    DEFAULT_CONFIG.modes().clone()
}
