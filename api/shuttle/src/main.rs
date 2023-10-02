use actix_web::web;
use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

use api_lib::routes::{hello_world, ping, version};
use api_lib::{films, health};

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let film_repository = api_lib::film_repository::PostgresFilmRepository::new(pool);
    let film_repository = actix_web::web::Data::new(film_repository);
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(film_repository)
            .configure(health::service)
            .configure(films::service)
            .service(hello_world)
            .service(ping)
            .service(version);
    };

    Ok(config.into())
}
