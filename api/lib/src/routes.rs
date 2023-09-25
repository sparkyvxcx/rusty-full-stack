use actix_web::get;
use actix_web::web;
use sqlx::PgPool;

#[get("/")]
pub async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/ping")]
pub async fn ping() -> &'static str {
    "PONG"
}

#[get("/version")]
pub async fn version(pool: web::Data<PgPool>) -> String {
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
