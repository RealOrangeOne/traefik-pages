use constant_time_eq::constant_time_eq;

use actix_web::dev::RequestHead;
use actix_web::guard::Guard;
use actix_web::http::header::AUTHORIZATION;
use actix_web_httpauth::headers::authorization::Basic;
use actix_web_httpauth::headers::authorization::Scheme;

pub struct BasicAuthGuard {
    password: String,
}

impl Guard for BasicAuthGuard {
    fn check(&self, request: &RequestHead) -> bool {
        match request
            .headers()
            .get(AUTHORIZATION)
            .and_then(|header_value| Basic::parse(header_value).ok())
        {
            Some(credentials) => {
                credentials.password().is_none()
                    && constant_time_eq(self.password.as_bytes(), credentials.user_id().as_bytes())
            }
            None => false,
        }
    }
}

impl BasicAuthGuard {
    pub fn new(password: &str) -> BasicAuthGuard {
        BasicAuthGuard {
            password: password.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::configure_app;
    use crate::test_utils::{auth_credentials, get_test_settings, TEST_PASSWORD};
    use actix_web::http::header;
    use actix_web::{test, App};

    use crate::routes::INTERNAL_ROUTE_PREFIX;
    use actix_web_httpauth::headers::authorization::Basic;

    fn get_test_path() -> String {
        format!("/{}/provider", INTERNAL_ROUTE_PREFIX)
    }

    #[tokio::test]
    async fn test_requires_auth_header() {
        let mut app =
            test::init_service(App::new().configure(|cfg| configure_app(cfg, get_test_settings())))
                .await;
        let request = test::TestRequest::get().uri(&get_test_path()).to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), 404);
    }

    #[tokio::test]
    async fn test_authed() {
        let mut app =
            test::init_service(App::new().configure(|cfg| configure_app(cfg, get_test_settings())))
                .await;
        let request = test::TestRequest::get()
            .uri(&get_test_path())
            .header(header::AUTHORIZATION, auth_credentials())
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_incorrect_username() {
        let mut app =
            test::init_service(App::new().configure(|cfg| configure_app(cfg, get_test_settings())))
                .await;
        let request = test::TestRequest::get()
            .uri(&get_test_path())
            .header(
                header::AUTHORIZATION,
                Basic::new("some username", Some("some password")),
            )
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), 404);
    }

    #[tokio::test]
    async fn test_password() {
        let mut app =
            test::init_service(App::new().configure(|cfg| configure_app(cfg, get_test_settings())))
                .await;
        let request = test::TestRequest::get()
            .uri(&get_test_path())
            .header(
                header::AUTHORIZATION,
                Basic::new(TEST_PASSWORD, Some("some password")),
            )
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), 404);
    }
}
