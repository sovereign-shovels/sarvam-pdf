use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub endpoint: Option<String>,
    pub api_key_env_var: Option<String>,
    pub model: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let mut merged = Self::default();

        if let Some(path) = Self::path() {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(cfg) = toml::from_str::<Config>(&content) {
                    merged.endpoint = cfg.endpoint.or(merged.endpoint);
                    merged.api_key_env_var = cfg.api_key_env_var.or(merged.api_key_env_var);
                    merged.model = cfg.model.or(merged.model);
                }
            }
        }

        if let Ok(v) = env::var("SARVAM_PDF_ENDPOINT") {
            merged.endpoint = Some(v);
        }
        if let Ok(v) = env::var("SARVAM_PDF_API_KEY_ENV") {
            merged.api_key_env_var = Some(v);
        }
        if let Ok(v) = env::var("SARVAM_PDF_MODEL") {
            merged.model = Some(v);
        }

        if merged.endpoint.is_none() {
            merged.endpoint = Some("https://api.sarvam.ai/translate".into());
        }

        merged
    }

    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("sarvam-pdf").join("config.toml"))
    }

    pub fn api_key(&self) -> Option<String> {
        self.api_key_env_var.as_ref().and_then(|k| env::var(k).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = Config::load();
        assert!(cfg.endpoint.is_some());
    }
}
