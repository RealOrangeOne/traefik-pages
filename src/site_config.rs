use serde_derive::Deserialize;
use std::path::PathBuf;
use tokio::fs;
use toml::from_str;

pub const CONFIG_FILENAME: &str = "pages.toml";

fn default_true() -> bool {
    true
}

#[derive(Default, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::current_dir;

    fn get_example_dir() -> PathBuf {
        current_dir().unwrap().join("example/sites")
    }

    #[tokio::test]
    async fn test_parse() {
        SiteConfig::new(get_example_dir().join("localhost").join(CONFIG_FILENAME)).await;
    }

    #[test]
    fn test_defaults() {
        let site_config: SiteConfig = from_str("").unwrap();

        assert!(site_config.dir_index);
    }
}
