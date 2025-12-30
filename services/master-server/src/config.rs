use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct MasterConfig {
    pub bind_addr: String,
    pub data_dir: String,
}

impl Default for MasterConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:4000".into(),
            data_dir: "./data/master".into(),
        }
    }
}

impl MasterConfig {
    pub fn from_env() -> Self {
        let mut cfg = MasterConfig::default();
        if let Ok(addr) = env::var("MASTER_BIND") {
            cfg.bind_addr = addr;
        }
        if let Ok(dir) = env::var("MASTER_DATA_DIR") {
            cfg.data_dir = dir;
        }
        cfg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_and_env() {
        let d = MasterConfig::default();
        assert!(d.data_dir.contains("data"));
        env::set_var("MASTER_BIND", "0.0.0.0:1234");
        let e = MasterConfig::from_env();
        assert_eq!(e.bind_addr, "0.0.0.0:1234");
    }
}
