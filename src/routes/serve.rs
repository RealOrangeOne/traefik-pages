use crate::config::Config;
use crate::files::normalize_path;
use crate::site::Site;
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

fn get_path(request: &HttpRequest) -> String {
    let original_path = request.match_info().get("path").unwrap().to_owned();
    normalize_path(original_path)
}

pub async fn serve_file(req: HttpRequest, config: web::Data<Config>) -> HttpResponse {
    let site = match Site::from_hostname(&config.sites_root, get_hostname(&req)).await {
        Some(s) => s,
        None => return HttpResponse::NotFound().finish(),
    };

    let file_path = match site.get_file(get_path(&req)).await {
        Ok(p) => p,
        Err(_) => return HttpResponse::NotFound().finish(),
    };
    NamedFile::open(file_path)
        .expect("Failed to open file")
        .disable_content_disposition()
        .into_response(&req)
        .expect("Failed to turn file into response")
}
