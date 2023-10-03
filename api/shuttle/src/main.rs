use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

use api_lib::film_repository::PostgresFilmRepository;
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

    let film_repo = api_lib::film_repository::PostgresFilmRepository::new(pool);
    let film_repo = web::Data::new(film_repo);
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(film_repo)
            .configure(health::service)
            .configure(films::service::<PostgresFilmRepository>)
            .service(hello_world)
            .service(ping)
            .service(version);
    };

    Ok(config.into())
}
