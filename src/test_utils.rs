use crate::settings::Settings;
use std::env::current_dir;
use std::path::PathBuf;

pub fn get_example_dir() -> PathBuf {
    current_dir().unwrap().join("example/sites")
}

pub fn get_test_settings() -> Settings {
    Settings {
        sites_root: get_example_dir(),
        traefik_service: String::from("traefik-service@docker"),
        traefik_cert_resolver: Some(String::from("le")),
        auth_password: String::default(),
        deny_prefixes: Vec::new(),
    }
}
