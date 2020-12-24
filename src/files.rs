use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::canonicalize;

#[inline]
pub async fn safe_join(base: impl AsRef<Path>, second: impl AsRef<Path>) -> io::Result<PathBuf> {
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

pub fn normalize_path(original_path: String) -> String {
    if original_path.is_empty() || original_path.ends_with('/') {
        return original_path + "index.html";
    }
    original_path
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::current_dir;

    fn get_example_dir() -> PathBuf {
        current_dir().unwrap().join("example")
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("foo/".into()), "foo/index.html");
        assert_eq!(normalize_path("foo.html".into()), "foo.html");
        assert_eq!(normalize_path("".into()), "index.html");
    }

    #[tokio::test]
    async fn test_safe_join_success() {
        assert_eq!(
            safe_join(get_example_dir(), "localhost").await.unwrap(),
            get_example_dir().join("localhost")
        );
    }

    #[tokio::test]
    async fn test_safe_join_fail() {
        let join_err = safe_join(get_example_dir(), "localhost2").await;
        assert!(join_err.is_err());
        assert_eq!(join_err.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_resolve_fail_traversal() {
        let resolve_err = safe_join(get_example_dir(), "../Cargo.toml")
            .await
            .unwrap_err();
        assert_eq!(resolve_err.kind(), io::ErrorKind::InvalidInput);
    }
}
