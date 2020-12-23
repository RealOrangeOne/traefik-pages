use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::env;
use std::path::PathBuf;
use std::process::exit;

mod config;
mod files;
mod serve;
mod utils;

fn get_sites_root() -> Option<PathBuf> {
    let sites_root_env = env::var("SITES_ROOT").ok()?;
    PathBuf::from(sites_root_env).canonicalize().ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let sites_root = match get_sites_root() {
        Some(p) => p,
        None => {
            log::error!("Invalid sites root");
            exit(1);
        }
    };

    let app_config = config::Config { sites_root };

    let local = tokio::task::LocalSet::new();
    let sys = actix_web::rt::System::run_in_tokio("server", &local);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(app_config.clone())
            .route("/{path:.*}", web::route().to(serve::serve_file))
    })
    .workers(utils::get_workers())
    .bind(format!("0.0.0.0:{}", utils::get_port()))?
    .run()
    .await?;

    sys.await?;

    Ok(())
}
