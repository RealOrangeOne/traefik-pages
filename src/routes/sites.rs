use crate::settings::Settings;
use crate::site::Site;
use actix_web::{web, HttpResponse};

pub async fn sites_list(settings: web::Data<Settings>) -> HttpResponse {
    let sites = match settings.discover_sites().await {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let hostnames = sites
        .iter()
        .map(Site::get_hostname)
        .collect::<Vec<String>>();

    return HttpResponse::Ok().body(hostnames.join("\n") + "\n");
}
