use crate::settings::{Settings, RETRY_COUNT};
use crate::site::Site;
use actix_web::{web, HttpResponse};
use serde_json::{json, Value};
use std::collections::HashMap;

const DEFAULT_MIDDLEWARE_NAME: &str = "tp-default";

fn get_router_name(site: &Site) -> String {
    site.get_hostname().replace(".", "-")
}

/// Routers look funny, so no point defining as a struct
fn serialize_router(site: &Site, settings: &Settings) -> Value {
    let mut router = json!({
        "rule": format!("Host(`{}`)", site.get_hostname()),
        "service": &settings.traefik_service,
        "middlewares": [DEFAULT_MIDDLEWARE_NAME]
    });
    if let Some(cert_resolver) = &settings.traefik_cert_resolver {
        router.as_object_mut().unwrap().insert(
            String::from("tls"),
            json!({ "certResolver": cert_resolver }),
        );
    }
    router
}

fn get_middleware() -> Value {
    json!({
        DEFAULT_MIDDLEWARE_NAME: {
            "chain": {
                "middlewares": [
                    "tp-retry"
                ]
            }
        },
        "tp-retry": {
            "retry": {
                "attempts": RETRY_COUNT
            }
        }
    })
}

pub async fn traefik_provider(settings: web::Data<Settings>) -> HttpResponse {
    let sites = match settings.discover_sites().await {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let routers: HashMap<String, Value> = sites
        .iter()
        .map(|s| (get_router_name(s), serialize_router(s, &settings)))
        .collect();

    HttpResponse::Ok().json(json!({
        "http": {
            "routers": routers,
            "middlewares": get_middleware()
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::site::Site;
    use crate::test_utils::{get_example_dir, get_test_settings};

    #[test]
    fn test_default_middleware() {
        let middleware = get_middleware();
        let default_middleware = &middleware[DEFAULT_MIDDLEWARE_NAME];
        let chain_middlewares = default_middleware["chain"]["middlewares"]
            .as_array()
            .unwrap();
        assert_eq!(chain_middlewares.len(), 1);
        for m in chain_middlewares.iter() {
            assert!(middleware.get(m.as_str().unwrap()).is_some());
        }
    }

    #[tokio::test]
    async fn test_router_name() {
        let example_site = Site::new(get_example_dir().join("localhost")).await;
        assert_eq!(get_router_name(&example_site), "localhost");

        let example_site_2 = Site::new(get_example_dir().join("site1.localhost")).await;
        assert_eq!(get_router_name(&example_site_2), "site1-localhost");
    }

    #[tokio::test]
    async fn test_serialize_router() {
        let settings = get_test_settings();
        let example_site = settings.site_from_hostname("localhost").await.unwrap();
        assert_eq!(
            serialize_router(&example_site, &settings),
            json!({
                "middlewares": [DEFAULT_MIDDLEWARE_NAME],
                "rule": "Host(`localhost`)",
                "service": "traefik-service@docker",
                "tls": {
                    "certResolver": "le"
                }
            })
        );
    }
}
