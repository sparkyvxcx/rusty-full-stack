use actix_web::web;
use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

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

#[get("/version")]
async fn version(pool: web::Data<PgPool>) -> String {
    match get_db_version(&pool).await {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}

async fn get_db_version(pool: &PgPool) -> Result<String, sqlx::Error> {
    tracing::info!("Getting version");
    // let version_query = "SHOW server_version";
    let version_query = "SELECT version()";
    sqlx::query_scalar(version_query).fetch_one(pool).await
}

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
