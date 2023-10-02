use actix_web::web::{self, ServiceConfig};
use actix_web::HttpResponse;
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use crate::film_repository::FilmRepository;

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

pub async fn get_films(repo: web::Data<Box<dyn FilmRepository>>) -> HttpResponse {
    tracing::info!("Getting a list of films");

    match repo.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

pub async fn get_film(
    repo: web::Data<Box<dyn FilmRepository>>,
    film_id: web::Path<Uuid>,
) -> HttpResponse {
    tracing::info!("Getting a specific film");

    match repo.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::NotFound().body(format!("Film with id {} Not found", film_id)),
    }
}

pub async fn post_film(
    repo: web::Data<Box<dyn FilmRepository>>,
    film: web::Json<CreateFilm>,
) -> HttpResponse {
    match repo.create_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::UnprocessableEntity().body(format!("Internal server error: {:?}", e))
        }
    }
}

pub async fn put_film(
    repo: web::Data<Box<dyn FilmRepository>>,
    film: web::Json<Film>,
) -> HttpResponse {
    match repo.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

pub async fn delete_film(
    repo: web::Data<Box<dyn FilmRepository>>,
    film_id: web::Path<Uuid>,
) -> HttpResponse {
    tracing::info!("Deleting a specific film");

    match repo.delete_film(&film_id).await {
        Ok(film_id) => HttpResponse::Ok().json(film_id),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}
