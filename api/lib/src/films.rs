use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(get_films))
            .route("/{film_id}", web::get().to(get_film))
            .route("", web::post().to(post_film))
            .route("", web::put().to(put_film))
            .route("/{film_id}", web::delete().to(delete_film)),
    );
}

pub async fn get_films() -> HttpResponse {
    tracing::info!("Getting a list of films");

    HttpResponse::Ok().finish()
}

pub async fn get_film() -> HttpResponse {
    tracing::info!("Getting a specific film");

    HttpResponse::Ok().finish()
}

pub async fn post_film() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn put_film() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn delete_film() -> HttpResponse {
    HttpResponse::Ok().finish()
}
