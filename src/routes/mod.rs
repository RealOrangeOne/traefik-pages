use actix_web::{web, Scope};
mod health;
mod serve;
mod sites;
mod traefik;
use crate::auth::BasicAuthGuard;
use crate::config::Config;

pub const INTERNAL_ROUTE_PREFIX: &str = ".traefik-pages";

fn get_internal_routes(config: &Config) -> Scope {
    web::scope(INTERNAL_ROUTE_PREFIX)
        .guard(BasicAuthGuard::new(&config.auth_password))
        .route("/health", web::route().to(health::health))
        .route("/sites", web::get().to(sites::sites_list))
        .route("/provider", web::get().to(traefik::traefik_provider))
}

pub fn get_routes(config: &Config) -> Scope {
    web::scope("")
        .service(get_internal_routes(config))
        // This must go at the end
        .route("/{path:.*}", web::route().to(serve::serve_file))
}
