use crate::files::safe_join;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct Site {
    root: PathBuf,
}

impl From<PathBuf> for Site {
    fn from(path: PathBuf) -> Self {
        debug_assert!(path.is_dir());

        Site { root: path }
    }
}

impl Site {
    pub fn get_hostname(&self) -> String {
        self.root.file_name().unwrap().to_str().unwrap().to_owned()
    }

    pub async fn from_hostname(sites_root: impl AsRef<Path>, hostname: String) -> Option<Site> {
        let site_root = safe_join(sites_root, hostname).await.ok()?;
        Some(Site::from(site_root))
    }

    pub async fn discover_all(sites_root: impl AsRef<Path>) -> io::Result<Vec<Site>> {
        let mut sites = vec![];

        let mut entries = fs::read_dir(sites_root).await?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.path().is_dir() {
                sites.push(Site::from(entry.path()))
            }
        }

        Ok(sites)
    }

    pub async fn get_file(&self, path: impl AsRef<Path>) -> io::Result<PathBuf> {
        safe_join(&self.root, path).await
    }

    pub fn router_name(&self) -> String {
        format!("router-{}", self.get_hostname().replace(".", "-"))
    }

    pub fn service_name(&self) -> String {
        format!("service-{}", self.get_hostname().replace(".", "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::current_dir;

    fn get_example_dir() -> PathBuf {
        current_dir().unwrap().join("example")
    }

    #[tokio::test]
    async fn test_discover_all() {
        let sites = Site::discover_all(get_example_dir()).await.unwrap();
        assert_eq!(sites.len(), 1);
        assert_eq!(
            sites
                .iter()
                .map(Site::get_hostname)
                .collect::<Vec<String>>(),
            vec!["localhost"]
        );
    }

    #[tokio::test]
    async fn test_get_file() {
        let site = Site::from(get_example_dir().join("localhost"));

        assert!(site.get_file("index.html").await.is_ok());
        assert!(site.get_file("missing.html").await.is_err());
    }

    #[test]
    fn test_from_path() {
        let site = Site::from(get_example_dir().join("localhost"));
        assert_eq!(site.get_hostname(), "localhost")
    }
}
