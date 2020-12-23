use crate::config::Config;
use crate::files::resolve_file;
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

    if original_path == "" || original_path.ends_with('/') {
        return original_path + "index.html";
    }
    original_path
}

pub async fn serve_file(req: HttpRequest, config: web::Data<Config>) -> HttpResponse {
    let hostname = get_hostname(&req);
    let path = get_path(&req);
    let file_path = match resolve_file(&config.sites_root, hostname, path).await {
        Ok(p) => p,
        Err(_) => return HttpResponse::NotFound().finish(),
    };
    NamedFile::open(file_path)
        .expect("Failed to open file")
        .disable_content_disposition()
        .into_response(&req)
        .expect("Failed to turn file into response")
}
