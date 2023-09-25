use actix_web::{get, HttpResponse};

#[get("/health_check")]
pub async fn health_check() -> HttpResponse {
    tracing::info!("Getting health check");

    HttpResponse::Ok()
        .append_header(("version", "v0.0.1"))
        .finish()
}
