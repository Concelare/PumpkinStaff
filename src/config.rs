use std::ops::Deref;
use std::path::PathBuf;
use std::sync::OnceLock;
use serde::{Deserialize, Serialize};

pub static CONFIG: Ref = Ref(OnceLock::new());

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct Config {
    pub mode: SecurityMode,
    pub redb_path: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Eq, PartialEq)]
pub enum SecurityMode {
    Password,
    TwoFactor,
    None,
}


impl Config {

    pub fn init(path: &PathBuf) {
        let config = Config::load(path);
        assert!(CONFIG.0.set(config).is_ok());
    }
    pub fn load(path: &PathBuf) -> Self {
        let config: Config = toml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        config
    }
}

pub struct Ref(OnceLock<Config>);

impl Deref for Ref {
    type Target = Config;
    fn deref(&self) -> &Self::Target {
        self.0.get().unwrap()
    }
}