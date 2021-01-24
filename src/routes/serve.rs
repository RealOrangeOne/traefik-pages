use crate::files::normalize_path;
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

fn get_path(request: &HttpRequest, dir_index: bool) -> String {
    let original_path = request.match_info().get("path").unwrap();
    if dir_index {
        normalize_path(original_path)
    } else {
        original_path.to_owned()
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

    let url_path = get_path(&req, site.config.dir_index);

    if settings
        .deny_prefixes
        .iter()
        .any(|prefix| url_path.starts_with(prefix))
    {
        return HttpResponse::NotFound().finish();
    }

    match site.get_file(&url_path).await {
        Ok(p) => NamedFile::open(p)
            .expect("Failed to open file")
            .disable_content_disposition()
            .into_response(&req)
            .expect("Failed to turn file into response"),
        Err(_) => return HttpResponse::NotFound().finish(),
    }
}
