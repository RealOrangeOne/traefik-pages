use actix_web::{web, Scope};
mod health;
mod serve;
mod sites;
mod traefik;
use crate::auth::BasicAuthGuard;
use crate::settings::Settings;

pub const INTERNAL_ROUTE_PREFIX: &str = ".traefik-pages";

fn get_internal_routes(settings: &Settings) -> Scope {
    web::scope(INTERNAL_ROUTE_PREFIX)
        .guard(BasicAuthGuard::new(&settings.auth_password))
        .route("/health", web::route().to(health::health))
        .route("/sites", web::get().to(sites::sites_list))
        .route("/provider", web::get().to(traefik::traefik_provider))
}

pub fn get_routes(settings: &Settings) -> Scope {
    web::scope("")
        .service(get_internal_routes(settings))
        // These must go at the end
        .route("/{path:.*}", web::get().to(serve::serve_file))
        .route("/{path:.*}", web::head().to(serve::serve_file))
}
