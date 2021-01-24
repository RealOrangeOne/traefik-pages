use std::path::PathBuf;

pub const RETRY_COUNT: u8 = 4;

#[derive(Clone)]
pub struct Settings {
    pub sites_root: PathBuf,
    pub traefik_service: String,
    pub traefik_cert_resolver: Option<String>,
    pub auth_password: String,
    pub deny_prefixes: Vec<String>,
}
