use crate::settings::Settings;
use crate::site::is_valid_hostname;
use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse};

fn get_hostname(request: &HttpRequest) -> String {
    let conn_info = request.connection_info();
    let host = conn_info.host();
    match host.find(':') {
        None => host.into(),
        Some(i) => host.split_at(i).0.into(),
    }
}

pub async fn serve_file(req: HttpRequest, settings: web::Data<Settings>) -> HttpResponse {
    let hostname = get_hostname(&req);
    if !is_valid_hostname(&hostname) {
        return HttpResponse::NotFound().finish();
    }
    let site = match settings.site_from_hostname(&hostname).await {
        Some(s) => s,
        None => return HttpResponse::NotFound().finish(),
    };

    let url_path = req.path().trim_start_matches('/');

    if settings
        .deny_prefixes
        .iter()
        .any(|prefix| url_path.starts_with(prefix))
    {
        return HttpResponse::NotFound().finish();
    }

    match site.get_file(&url_path, site.config.dir_index).await {
        Ok(p) => NamedFile::open(p)
            .expect("Failed to open file")
            .disable_content_disposition()
            .into_response(&req)
            .expect("Failed to turn file into response"),
        Err(_) => return HttpResponse::NotFound().finish(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::app::configure_app;
    use crate::test_utils::get_test_settings;
    use actix_web::http::header;
    use actix_web::web::Bytes;
    use actix_web::{test, App};

    async fn get_content_at_path(hostname: &str, path: &str) -> Bytes {
        let mut app =
            test::init_service(App::new().configure(|cfg| configure_app(cfg, get_test_settings())))
                .await;
        let request = test::TestRequest::get()
            .uri(path)
            .header(header::HOST, hostname)
            .to_request();
        test::read_response(&mut app, request).await
    }

    #[test]
    fn test_get_hostname() {
        let request = test::TestRequest::get()
            .uri("/")
            .header(header::HOST, "localhost")
            .to_http_request();

        assert_eq!(get_hostname(&request), "localhost");
    }

    #[test]
    fn test_get_hostname_with_port() {
        let request = test::TestRequest::get()
            .uri("/")
            .header(header::HOST, "localhost:5000")
            .to_http_request();

        assert_eq!(get_hostname(&request), "localhost");
    }

    #[tokio::test]
    async fn test_serve_correct_files() {
        assert_eq!(
            get_content_at_path("localhost", "/").await,
            Bytes::from_static(b"localhost index\n")
        );
        assert_eq!(
            get_content_at_path("localhost", "/index.html").await,
            Bytes::from_static(b"localhost index\n")
        );
        assert_eq!(
            get_content_at_path("localhost", "/sub/").await,
            Bytes::from_static(b"localhost subdir\n")
        );
        assert_eq!(
            get_content_at_path("localhost", "/sub").await,
            Bytes::from_static(b"localhost subdir\n")
        );
        assert_eq!(
            get_content_at_path("localhost", "/sub/index.html").await,
            Bytes::from_static(b"localhost subdir\n")
        );
        assert_eq!(
            get_content_at_path("site1.localhost", "/").await,
            Bytes::from_static(b"Site 1\n")
        );
    }

    #[tokio::test]
    async fn test_unknown_hostname() {
        let mut app =
            test::init_service(App::new().configure(|cfg| configure_app(cfg, get_test_settings())))
                .await;
        let request = test::TestRequest::get()
            .uri("/")
            .header(header::HOST, "unknown")
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), 404);
    }

    #[tokio::test]
    async fn test_unknown_path() {
        let mut app =
            test::init_service(App::new().configure(|cfg| configure_app(cfg, get_test_settings())))
                .await;
        let request = test::TestRequest::get()
            .uri("/unknown.html")
            .header(header::HOST, "localhost")
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), 404);
    }

    #[tokio::test]
    async fn test_invalid_hostname() {
        let mut app =
            test::init_service(App::new().configure(|cfg| configure_app(cfg, get_test_settings())))
                .await;
        let request = test::TestRequest::get()
            .uri("/")
            .header(header::HOST, ".localhost")
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), 404);
    }
}
