use crate::site::Site;
use std::path::PathBuf;
pub const RETRY_COUNT: u8 = 4;
use std::io;

#[derive(Clone)]
pub struct Settings {
    pub sites_root: PathBuf,
    pub traefik_service: String,
    pub traefik_cert_resolver: Option<String>,
    pub auth_password: String,
    pub deny_prefixes: Vec<String>,
}

impl Settings {
    pub async fn discover_sites(&self) -> io::Result<Vec<Site>> {
        Site::discover_all(&self.sites_root).await
    }

    pub async fn site_from_hostname(&self, hostname: &str) -> Option<Site> {
        Site::from_hostname(&self.sites_root, hostname).await
    }
}
