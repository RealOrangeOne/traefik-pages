use crate::config::Config;
use crate::sites::{get_hostname, get_sites};
use actix_web::{web, HttpResponse};

pub async fn sites_list(config: web::Data<Config>) -> HttpResponse {
    let sites = match get_sites(&config.sites_root).await {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let hostnames = sites.iter().map(get_hostname).collect::<Vec<String>>();

    return HttpResponse::Ok().body(hostnames.join("\n") + "\n");
}
