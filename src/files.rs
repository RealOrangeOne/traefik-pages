use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::canonicalize;

#[inline]
async fn safe_join(base: impl AsRef<Path>, second: impl AsRef<Path>) -> io::Result<PathBuf> {
    let joined = canonicalize(base.as_ref().join(&second)).await?;

    if !joined.starts_with(&base) {
        return io::Result::Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Resulting path must be a child of {}. Got {}.",
                base.as_ref().display(),
                joined.display()
            ),
        ));
    }

    io::Result::Ok(joined)
}

pub async fn resolve_file(
    sites_root: impl AsRef<Path>,
    hostname: String,
    path: String,
) -> io::Result<PathBuf> {
    let site_root = safe_join(sites_root, hostname).await?;
    safe_join(site_root, path).await
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::current_dir;

    fn get_example_dir() -> PathBuf {
        current_dir().unwrap().join("example")
    }

    #[tokio::test]
    async fn test_resolve_success() {
        assert_eq!(
            resolve_file(get_example_dir(), "localhost".into(), "index.html".into())
                .await
                .unwrap(),
            get_example_dir().join("localhost/index.html")
        );
    }

    #[tokio::test]
    async fn test_resolve_fail() {
        assert!(
            resolve_file(get_example_dir(), "localhost".into(), "index2.html".into())
                .await
                .is_err()
        );
        assert!(
            resolve_file(get_example_dir(), "localhost2".into(), "index.html".into())
                .await
                .is_err()
        );
        let resolve_err = resolve_file(get_example_dir(), "localhost".into(), "index2.html".into())
            .await
            .unwrap_err();
        assert_eq!(resolve_err.kind(), io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_resolve_fail_traversal() {
        let resolve_err = resolve_file(
            get_example_dir(),
            "localhost".into(),
            "../../Cargo.toml".into(),
        )
        .await
        .unwrap_err();
        assert_eq!(resolve_err.kind(), io::ErrorKind::InvalidInput);
    }
}
