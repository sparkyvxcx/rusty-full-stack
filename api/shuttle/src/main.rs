use actix_web::web;
use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::postgres::PgRow;
use sqlx::{Executor, PgPool, Row};

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
    get_db_version(&pool)
        .await
        .expect("Failed to get db version")
}

async fn get_db_version(pool: &PgPool) -> Result<String, sqlx::Error> {
    let version_query = "SHOW server_version;";
    match sqlx::query(version_query)
        .map(|row: PgRow| -> String { row.get(0) })
        .fetch_one(pool)
        .await
    {
        Ok(row) => Ok(row),
        Err(_) => Err(sqlx::Error::RowNotFound),
    }
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
            .app_data(db_pool.clone());
    };

    Ok(config.into())
}
