use actix_web::HttpResponse;

pub async fn health() -> HttpResponse {
    return HttpResponse::Ok().finish();
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::http::StatusCode;

    #[tokio::test]
    async fn test_health() {
        let response = health().await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
