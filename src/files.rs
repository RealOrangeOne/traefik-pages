use std::io;
use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn is_dir(path: impl AsRef<Path>) -> bool {
    match fs::metadata(path).await {
        Ok(m) => m.is_dir(),
        Err(_) => false,
    }
}

pub async fn ensure_file(maybe_path: io::Result<PathBuf>) -> io::Result<PathBuf> {
    if let Ok(ref path) = maybe_path {
        if is_dir(path).await {
            return io::Result::Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                String::from("Is a directory"),
            ));
        }
    }
    maybe_path
}

pub async fn safe_join(base: impl AsRef<Path>, second: impl AsRef<Path>) -> io::Result<PathBuf> {
    let joined = fs::canonicalize(base.as_ref().join(&second)).await?;

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

pub async fn handle_index(path: impl AsRef<Path>, index_file_name: &str) -> io::Result<PathBuf> {
    if is_dir(&path).await {
        safe_join(path, index_file_name).await
    } else {
        Ok(path.as_ref().to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::get_example_dir;

    #[tokio::test]
    async fn test_safe_join_success() {
        assert_eq!(
            safe_join(get_example_dir(), "localhost").await.unwrap(),
            get_example_dir().join("localhost")
        );
    }

    #[tokio::test]
    async fn test_handle_index() {
        assert_eq!(
            handle_index(get_example_dir().join("localhost/sub"), "index.html")
                .await
                .unwrap(),
            get_example_dir().join("localhost/sub/index.html")
        );
        assert_eq!(
            handle_index(get_example_dir().join("localhost/index.html"), "index.html")
                .await
                .unwrap(),
            get_example_dir().join("localhost/index.html")
        );
        assert!(
            handle_index(get_example_dir().join("localhost/sub"), "index.php")
                .await
                .is_err()
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
        let resolve_err = safe_join(get_example_dir(), "../../Cargo.toml")
            .await
            .unwrap_err();
        assert_eq!(resolve_err.kind(), io::ErrorKind::InvalidInput);
    }
}
