pub const DEFAULT_CONFIG_YAML: &str = include_str!("config.yml");

use crate::config::Config;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_CONFIG: Config = serde_yaml::from_str(DEFAULT_CONFIG_YAML).unwrap();
}
