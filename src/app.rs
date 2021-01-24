use crate::routes;
use crate::settings::Settings;
use crate::VERSION;
use actix_web::http::header;
use actix_web::middleware::DefaultHeaders;
use actix_web::web::ServiceConfig;

pub fn configure_app(cfg: &mut ServiceConfig, settings: Settings) {
    let service = routes::get_routes(&settings).wrap(
        DefaultHeaders::new()
            .header(header::SERVER, format!("traefik-pages {}", VERSION))
            .header(header::CACHE_CONTROL, "max-age=0, must-revalidate, public"),
    );
    cfg.data(settings).service(service);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::HeaderValue;
    use actix_web::web::Bytes;
    use actix_web::{test, App};
    use std::env::current_dir;

    fn get_settings() -> Settings {
        Settings {
            sites_root: current_dir().unwrap().join("example/sites"),
            traefik_service: String::from("traefik-service@docker"),
            traefik_cert_resolver: Some(String::from("le")),
            auth_password: String::default(),
            deny_prefixes: Vec::new(),
        }
    }

    #[tokio::test]
    async fn test_get_index() {
        let mut app =
            test::init_service(App::new().configure(|cfg| configure_app(cfg, get_settings())))
                .await;
        let request = test::TestRequest::get()
            .uri("/")
            .header(header::HOST, "localhost")
            .to_request();
        let data = test::read_response(&mut app, request).await;
        assert_eq!(data, Bytes::from_static(b"localhost index\n"));
    }

    #[tokio::test]
    async fn test_default_headers() {
        let mut app =
            test::init_service(App::new().configure(|cfg| configure_app(cfg, get_settings())))
                .await;
        let request = test::TestRequest::get()
            .uri("/")
            .header(header::HOST, "localhost")
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        assert_eq!(
            response.headers().get(header::SERVER).unwrap(),
            HeaderValue::from_str(&format!("traefik-pages {}", VERSION)).unwrap()
        );
        assert_eq!(
            response.headers().get(header::CACHE_CONTROL).unwrap(),
            HeaderValue::from_static("max-age=0, must-revalidate, public")
        );
    }
}
