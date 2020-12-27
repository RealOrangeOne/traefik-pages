use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;
use std::env;
use std::path::PathBuf;

mod config;
mod files;
mod routes;
mod site;
mod utils;

fn get_sites_root() -> PathBuf {
    let sites_root_env = utils::get_env_or_default("SITES_ROOT", None);
    match PathBuf::from(sites_root_env).canonicalize() {
        Ok(p) => p,
        Err(_) => utils::log_error_and_quit("Invalid site root."),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app_config = config::Config {
        sites_root: get_sites_root(),
        traefik_service: utils::get_env_or_default("TRAEFIK_SERVICE", None),
        traefik_cert_resolver: env::var("TRAEFIK_CERT_RESOLVER").ok(),
    };

    let local = tokio::task::LocalSet::new();
    let sys = actix_web::rt::System::run_in_tokio("server", &local);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(app_config.clone())
            .service(routes::get_routes())
    })
    .workers(utils::get_workers())
    .bind(format!("0.0.0.0:{}", utils::get_port()))?
    .run()
    .await?;

    sys.await?;

    Ok(())
}
