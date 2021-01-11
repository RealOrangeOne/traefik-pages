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
    pub fn new(password: String) -> BasicAuthGuard {
        BasicAuthGuard { password }
    }
}
