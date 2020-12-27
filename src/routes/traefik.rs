use crate::config::Config;
use crate::site::Site;
use actix_web::{web, HttpResponse};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

fn get_router_name(site: &Site) -> String {
    format!("router-{}", site.get_hostname().replace(".", "-"))
}

#[derive(Serialize, Debug)]
struct Router {
    rule: String,
    service: String,
}

impl Router {
    fn new(site: &Site, config: &Config) -> Self {
        Router {
            rule: format!("Host(`{}`)", site.get_hostname()),
            service: config.traefik_service.clone(),
        }
    }
}

pub async fn traefik_provider(config: web::Data<Config>) -> HttpResponse {
    let sites = match Site::discover_all(&config.sites_root).await {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let routers: HashMap<String, Router> = sites
        .iter()
        .map(|s| (get_router_name(s), Router::new(s, &config)))
        .collect();
    HttpResponse::Ok().json(json!({
        "http": {
            "routers": routers,
        }
    }))
}
