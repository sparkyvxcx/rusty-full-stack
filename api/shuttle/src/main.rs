use actix_web::web;
use actix_web::web::ServiceConfig;
use api_lib::health::health_check;
use api_lib::routes::{hello_world, ping, version};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let db_pool = web::Data::new(pool);
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
            .service(health_check)
            .service(ping)
            .service(version)
            .app_data(db_pool);
    };

    Ok(config.into())
}
