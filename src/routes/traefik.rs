use crate::config::Config;
use crate::site::Site;
use actix_web::{web, HttpResponse};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
struct Router {
    rule: String,
    service: String,
}

impl From<&Site> for Router {
    fn from(site: &Site) -> Self {
        Router {
            rule: site.get_hostname(),
            service: site.service_name(),
        }
    }
}

#[derive(Serialize, Debug)]
struct Service {}

impl From<&Site> for Service {
    fn from(_site: &Site) -> Self {
        Service {}
    }
}

pub async fn traefik_provider(config: web::Data<Config>) -> HttpResponse {
    let sites = match Site::discover_all(&config.sites_root).await {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let routers: HashMap<String, Router> = sites
        .iter()
        .map(|s| (s.router_name(), Router::from(s)))
        .collect();
    let services: HashMap<String, Service> = sites
        .iter()
        .map(|s| (s.service_name(), Service::from(s)))
        .collect();
    HttpResponse::Ok().json(json!({
        "http": {
            "routers": routers,
            "services": services
        }
    }))
}
