use std::io;
use std::path::{Path, PathBuf};
use tokio::fs;

async fn get_sites(sites_root: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    let mut sites = vec![];

    let mut entries = fs::read_dir(sites_root).await?;

    while let Some(entry) = entries.next_entry().await? {
        if entry.path().is_dir() {
            sites.push(entry.path())
        }
    }

    Ok(sites)
}

fn get_hostname(site_root: impl AsRef<Path>) -> String {
    site_root
        .as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::current_dir;

    fn get_example_dir() -> PathBuf {
        current_dir().unwrap().join("example")
    }

    #[tokio::test]
    async fn test_get_sites() {
        let sites = get_sites(get_example_dir()).await.unwrap();
        assert_eq!(sites.len(), 1);
        assert_eq!(
            sites.iter().map(get_hostname).collect::<Vec<String>>(),
            vec!["localhost"]
        );
    }

    #[tokio::test]
    async fn test_get_hostname() {
        assert_eq!(get_hostname("/foo/example.com"), "example.com");
        assert_eq!(get_hostname("example.com"), "example.com");
    }
}
