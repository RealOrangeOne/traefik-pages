use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub sites_root: PathBuf,
    pub traefik_service: String,
}
