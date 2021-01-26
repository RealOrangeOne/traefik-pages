use crate::files::handle_index;
use crate::files::{ensure_file, is_dir, safe_join};
use crate::site_config::{SiteConfig, CONFIG_FILENAME};
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs;
use url::Host;

pub fn is_valid_hostname(hostname: &str) -> bool {
    !hostname.starts_with('.') && Host::parse(hostname).is_ok()
}

pub struct Site {
    root: PathBuf,
    pub config: SiteConfig,
    config_path: Option<PathBuf>,
}

impl Site {
    pub async fn new(root: PathBuf) -> Self {
        debug_assert!(root.is_dir());

        let maybe_config_path = safe_join(&root, CONFIG_FILENAME).await.ok();

        let config = match maybe_config_path {
            Some(ref p) => SiteConfig::new(p).await,
            None => SiteConfig::default(),
        };

        Site {
            root,
            config,
            config_path: maybe_config_path,
        }
    }

    pub fn get_index_file(&self) -> Option<String> {
        if self.config.dir_index {
            Some(self.config.dir_index_name.clone())
        } else {
            None
        }
    }

    pub fn get_hostname(&self) -> String {
        self.root.file_name().unwrap().to_str().unwrap().to_owned()
    }

    pub async fn from_hostname(sites_root: impl AsRef<Path>, hostname: &str) -> Option<Site> {
        debug_assert!(is_valid_hostname(&hostname));
        let site_root = safe_join(sites_root, hostname).await.ok()?;
        Some(Site::new(site_root).await)
    }

    pub async fn discover_all(sites_root: impl AsRef<Path>) -> io::Result<Vec<Site>> {
        let mut sites = vec![];

        let mut entries = fs::read_dir(sites_root).await?;

        while let Some(entry) = entries.next_entry().await? {
            if is_dir(entry.path()).await {
                let site = Site::new(entry.path()).await;

                if is_valid_hostname(&site.get_hostname()) {
                    sites.push(site);
                }
            }
        }

        Ok(sites)
    }

    pub async fn get_file_for_path(&self, path: impl AsRef<Path>) -> io::Result<PathBuf> {
        let maybe_joined_path = safe_join(&self.root, path).await;
        if let Ok(joined_path) = maybe_joined_path {
            if is_dir(&joined_path).await {
                let index_file = match self.get_index_file() {
                    Some(index_file) => handle_index(&joined_path, &index_file).await,
                    None => Ok(joined_path),
                };
                return ensure_file(index_file).await;
            } else if let Some(ref config_path) = self.config_path {
                if &joined_path == config_path {
                    return io::Result::Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        String::from("File not found"),
                    ));
                }
            }
            return Ok(joined_path);
        }
        maybe_joined_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::get_example_dir;

    #[tokio::test]
    async fn test_discover_all() {
        let sites = Site::discover_all(get_example_dir()).await.unwrap();
        assert_eq!(sites.len(), 3);
        let site_hostnames = sites
            .iter()
            .map(Site::get_hostname)
            .collect::<Vec<String>>();
        assert!(site_hostnames.contains(&String::from("localhost")));
        assert!(site_hostnames.contains(&String::from("site1.localhost")));
        assert!(site_hostnames.contains(&String::from("no-index.localhost")));
    }

    #[tokio::test]
    async fn test_get_file_for_path() {
        let site = Site::new(get_example_dir().join("localhost")).await;

        assert!(site.get_file_for_path("index.html").await.is_ok());
        assert!(site.get_file_for_path("missing.html").await.is_err());

        assert_eq!(
            site.get_file_for_path("index.html").await.unwrap(),
            get_example_dir().join("localhost/index.html")
        );

        assert_eq!(
            site.get_file_for_path("sub").await.unwrap(),
            get_example_dir().join("localhost/sub/index.html")
        );

        assert_eq!(
            site.get_file_for_path("").await.unwrap(),
            get_example_dir().join("localhost/index.html")
        );
        assert!(site.get_file_for_path("sub-no-index").await.is_err());
        assert!(site.get_file_for_path(CONFIG_FILENAME).await.is_err());
    }

    #[tokio::test]
    async fn test_from_path() {
        let site = Site::new(get_example_dir().join("localhost")).await;
        assert_eq!(site.get_hostname(), "localhost")
    }

    #[test]
    fn test_is_valid_hostname() {
        assert!(is_valid_hostname("example.com"));
        assert!(is_valid_hostname("subdomain.example.com"));
        assert!(is_valid_hostname("example"));

        assert!(!is_valid_hostname(".example.com"));
        assert!(!is_valid_hostname("../site"));
    }
}
