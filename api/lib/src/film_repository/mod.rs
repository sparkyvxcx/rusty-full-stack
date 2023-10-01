use shared::models::Film;
use uuid::Uuid;

pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

#[async_trait::async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    async fn get_films(&self) -> FilmResult<Vec<Film>>;
    async fn get_film(&self, id: &Uuid) -> FilmResult<Film>;
    async fn create_film() -> FilmResult<Film>;
    async fn update_film() -> FilmResult<Film>;
    async fn delete_film() -> FilmResult<Uuid>;
}
