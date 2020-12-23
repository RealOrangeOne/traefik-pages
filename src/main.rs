use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;

mod files;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let local = tokio::task::LocalSet::new();
    let sys = actix_web::rt::System::run_in_tokio("server", &local);

    let server_res = HttpServer::new(move || App::new().wrap(Logger::default()))
        .workers(utils::get_workers())
        .bind(format!("0.0.0.0:{}", utils::get_port()))?
        .run()
        .await?;
    sys.await?;

    Ok(server_res)
}
