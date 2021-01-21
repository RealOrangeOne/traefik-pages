use crate::files::safe_join;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs;
use url::Host;

pub fn is_valid_hostname(hostname: &str) -> bool {
    !hostname.starts_with('.') && Host::parse(hostname).is_ok()
}

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

    pub async fn from_hostname(sites_root: impl AsRef<Path>, hostname: &str) -> Option<Site> {
        debug_assert!(is_valid_hostname(&hostname));
        let site_root = safe_join(sites_root, hostname).await.ok()?;
        Some(Site::from(site_root))
    }

    pub async fn discover_all(sites_root: impl AsRef<Path>) -> io::Result<Vec<Site>> {
        let mut sites = vec![];

        let mut entries = fs::read_dir(sites_root).await?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.path().is_dir() {
                let site = Site::from(entry.path());

                if is_valid_hostname(&site.get_hostname()) {
                    sites.push(site);
                }
            }
        }

        Ok(sites)
    }

    pub async fn get_file(&self, path: impl AsRef<Path>) -> io::Result<PathBuf> {
        safe_join(&self.root, path).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::current_dir;
    use std::fs::File;
    use std::io::Read;

    fn get_example_dir() -> PathBuf {
        current_dir().unwrap().join("example/sites")
    }

    #[tokio::test]
    async fn test_discover_all() {
        let sites = Site::discover_all(get_example_dir()).await.unwrap();
        assert_eq!(sites.len(), 2);
        let site_hostnames = sites
            .iter()
            .map(Site::get_hostname)
            .collect::<Vec<String>>();
        assert!(site_hostnames.contains(&String::from("localhost")));
        assert!(site_hostnames.contains(&String::from("site1.localhost")));
    }

    #[tokio::test]
    async fn test_get_file() {
        let site = Site::from(get_example_dir().join("localhost"));

        assert!(site.get_file("index.html").await.is_ok());
        assert!(site.get_file("missing.html").await.is_err());

        assert_eq!(
            site.get_file("index.html").await.unwrap(),
            get_example_dir().join("localhost/index.html")
        );
    }

    #[tokio::test]
    async fn test_get_file_content() {
        let site = Site::from(get_example_dir().join("localhost"));

        let mut file = File::open(site.get_file("index.html").await.unwrap()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert_eq!(contents, "localhost index\n");
    }

    #[test]
    fn test_from_path() {
        let site = Site::from(get_example_dir().join("localhost"));
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
