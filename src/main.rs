use actix_web::middleware::{Compress, Logger};
use actix_web::{App, HttpServer};
use env_logger::Env;
use std::env;
use std::path::PathBuf;

mod app;
mod auth;
mod files;
mod routes;
mod settings;
mod site;
mod site_config;
mod utils;

#[cfg(test)]
mod test_utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_sites_root() -> PathBuf {
    let sites_root_env = utils::get_env_or_default("SITES_ROOT", None);
    match PathBuf::from(sites_root_env).canonicalize() {
        Ok(p) => p,
        Err(_) => utils::log_error_and_quit("Invalid site root."),
    }
}

fn get_logger() -> Logger {
    let logger = Logger::new(r#"%{r}a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#);

    if env::var("LOG_INTERNAL").is_ok() {
        return logger;
    }

    logger.exclude_regex(format!(r#"\{}"#, routes::INTERNAL_ROUTE_PREFIX))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let settings = settings::Settings {
        sites_root: get_sites_root(),
        traefik_service: utils::get_env_or_default("TRAEFIK_SERVICE", None),
        traefik_cert_resolver: env::var("TRAEFIK_CERT_RESOLVER").ok(),
        auth_password: utils::get_env_or_default("AUTH_PASSWORD", None),
        deny_prefixes: utils::get_env_or_default("DENY_PREFIXES", Some(""))
            .split(',')
            .map(String::from)
            .filter(|s| !s.is_empty())
            .collect(),
    };

    let local = tokio::task::LocalSet::new();
    let sys = actix_web::rt::System::run_in_tokio("server", &local);

    HttpServer::new(move || {
        App::new()
            .configure(|cfg| app::configure_app(cfg, settings.clone()))
            .wrap(get_logger())
            .wrap(Compress::default())
    })
    .workers(utils::get_workers())
    .bind(format!("0.0.0.0:{}", utils::get_port()))?
    .run()
    .await?;

    sys.await?;

    Ok(())
}
