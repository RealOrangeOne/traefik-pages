use serde_derive::Deserialize;
use std::path::PathBuf;
use tokio::fs;
use toml::from_str;

pub const CONFIG_FILENAME: &str = "pages.toml";

fn default_true() -> bool {
    true
}

#[derive(Deserialize)]
pub struct SiteConfig {
    #[serde(default = "default_true")]
    pub dir_index: bool,
}

impl SiteConfig {
    pub async fn new(file: PathBuf) -> Self {
        debug_assert!(file.is_file());

        let contents = fs::read_to_string(file)
            .await
            .expect("Failed to read site config");

        from_str(&contents).expect("Failed to parse site config")
    }
}

impl Default for SiteConfig {
    fn default() -> Self {
        from_str("").expect("Default for site failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::get_example_dir;

    #[tokio::test]
    async fn test_parse() {
        SiteConfig::new(get_example_dir().join("localhost").join(CONFIG_FILENAME)).await;
    }

    #[test]
    fn test_defaults() {
        let site_config = SiteConfig::default();

        assert!(site_config.dir_index);
    }
}
