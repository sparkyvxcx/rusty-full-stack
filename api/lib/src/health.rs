use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check));
}

pub async fn health_check() -> HttpResponse {
    tracing::info!("Getting health check");

    HttpResponse::Ok()
        .append_header(("version", "v0.0.1"))
        .finish()
}
