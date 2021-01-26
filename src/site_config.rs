use serde_derive::Deserialize;
use std::path::Path;
use tokio::fs;
use toml::from_str;

pub const CONFIG_FILENAME: &str = "pages.toml";

fn default_true() -> bool {
    true
}

fn default_dir_index_name() -> String {
    String::from("index.html")
}

#[derive(Deserialize)]
pub struct SiteConfig {
    #[serde(default = "default_true")]
    pub dir_index: bool,

    #[serde(default = "default_dir_index_name")]
    pub dir_index_name: String,
}

impl SiteConfig {
    pub async fn new(file: impl AsRef<Path>) -> Self {
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
        assert_eq!(&site_config.dir_index_name, "index.html");
    }
}
