use crate::config::Config;
use crate::site::Site;
use actix_web::{web, HttpResponse};
use serde_json::{json, Value};
use std::collections::HashMap;

fn get_router_name(site: &Site) -> String {
    format!("router-{}", site.get_hostname().replace(".", "-"))
}

/// Routers look funny, so no point defining as a struct
fn serialize_router(site: &Site, config: &Config) -> Value {
    json!({
        "rule": format!("Host(`{}`)", site.get_hostname()),
        "service": &config.traefik_service
    })
}

pub async fn traefik_provider(config: web::Data<Config>) -> HttpResponse {
    let sites = match Site::discover_all(&config.sites_root).await {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let routers: HashMap<String, Value> = sites
        .iter()
        .map(|s| (get_router_name(s), serialize_router(s, &config)))
        .collect();
    HttpResponse::Ok().json(json!({
        "http": {
            "routers": routers,
        }
    }))
}
