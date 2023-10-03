use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;
use std::path::PathBuf;

use api_lib::film_repository::PostgresFilmRepository;
use api_lib::routes::{hello_world, ping, version};
use api_lib::{films, health};

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let film_repo = api_lib::film_repository::PostgresFilmRepository::new(pool);
    let film_repo = web::Data::new(film_repo);
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(film_repo)
                .configure(health::service)
                .configure(films::service::<PostgresFilmRepository>),
        )
        .service(hello_world)
        .service(ping)
        .service(version)
        .service(
            actix_files::Files::new("/", static_folder)
                .show_files_listing()
                .index_file("index.html"),
        );
    };

    Ok(config.into())
}
