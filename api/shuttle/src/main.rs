use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/health_check")]
async fn health_check() -> &'static str {
    "OK"
}

#[get("/ping")]
async fn ping() -> &'static str {
    "PONG"
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world).service(health_check).service(ping);
    };

    Ok(config.into())
}
