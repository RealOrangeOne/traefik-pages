use actix_web::{web, Scope};
mod health;
mod serve;
mod sites;

fn get_internal_routes() -> Scope {
    web::scope(".traefik-pages")
        .route("/health", web::route().to(health::health))
        .route("/sites", web::get().to(sites::sites_list))
}

pub fn get_routes() -> Scope {
    web::scope("")
        .service(get_internal_routes())
        // This must go at the end
        .route("/{path:.*}", web::route().to(serve::serve_file))
}
