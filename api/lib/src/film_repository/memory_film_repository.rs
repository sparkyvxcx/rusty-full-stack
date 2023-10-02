use super::{FilmRepository, FilmResult};
use shared::models::{CreateFilm, Film};
use std::{collections::HashMap, sync::RwLock};

pub struct MemoryFilmRepository {
    store: RwLock<HashMap<uuid::Uuid, Film>>,
}

impl MemoryFilmRepository {
    pub fn new() -> MemoryFilmRepository {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryFilmRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl FilmRepository for MemoryFilmRepository {
    async fn get_films(&self) -> FilmResult<Vec<Film>> {
        let result = self
            .store
            .read()
            .map(|films| films.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>())
            .map_err(|e| format!("An error occured while trying to read films store: {}", e));

        if result.is_err() {
            tracing::error!("Couldn't retrive a films");
        }

        result
    }

    async fn get_film(&self, film_id: &uuid::Uuid) -> FilmResult<Film> {
        let result = self
            .store
            .read()
            .map_err(|e| format!("An error occured while trying to read films store: {}", e))
            .and_then(|films| {
                films
                    .get(film_id)
                    .cloned()
                    .ok_or_else(|| format!("Couldn't find film: {}", film_id))
            });

        if result.is_err() {
            tracing::error!("Couldn't retrive a film with id {}", film_id);
        }

        result
    }

    async fn create_film(&self, create_film: &CreateFilm) -> FilmResult<Film> {
        match self.store.write() {
            Ok(mut films) => {
                let id = uuid::Uuid::new_v4();
                let utc_now = chrono::Utc::now();
                let new_film = Film {
                    id,
                    title: create_film.title.clone(),
                    director: create_film.director.clone(),
                    year: create_film.year,
                    poster: create_film.poster.clone(),
                    created_at: Some(utc_now),
                    updated_at: None,
                };
                films.insert(id, new_film.clone());
                tracing::trace!("Film with id {} successfully created", id);
                Ok(new_film)
            }
            Err(e) => {
                let err = format!("An error occured while trying to create film: {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }

    async fn update_film(&self, film: &Film) -> FilmResult<Film> {
        match self.store.write() {
            Ok(mut films) => {
                let utc_now = chrono::Utc::now();
                if let Some(the_film) = films.get_mut(&film.id) {
                    the_film.title = film.title.clone();
                    the_film.director = film.director.clone();
                    the_film.year = film.year;
                    the_film.poster = film.poster.clone();
                    the_film.updated_at = Some(utc_now);
                    Ok(the_film.clone())
                } else {
                    Err(format!("Film with id {} does not exist", film.id))
                }
            }
            Err(e) => {
                let err = format!("An error occured while trying to update film: {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }

    async fn delete_film(&self, film_id: &uuid::Uuid) -> FilmResult<uuid::Uuid> {
        match self.store.write() {
            Ok(mut films) => {
                films.remove(&film_id);
                Ok(film_id.to_owned())
            }
            Err(e) => {
                let err = format!("An error occured while trying to delete film: {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_empty_store() -> MemoryFilmRepository {
        MemoryFilmRepository::new()
    }

    fn generate_test_film(id: &'static str) -> Film {
        Film {
            id: uuid::Uuid::new_v4(),
            title: format!("title-{}", id),
            director: format!("director-{}", id),
            poster: format!("poster-{}", id),
            year: 2001,
            created_at: Some(chrono::Utc::now()),
            updated_at: None,
        }
    }

    fn generate_test_create_film(id: &'static str) -> CreateFilm {
        CreateFilm {
            title: format!("title-{}", id),
            director: format!("director-{}", id),
            poster: format!("poster-{}", id),
            year: 2001,
        }
    }

    #[actix_rt::test]
    async fn empty_store_will_return_empty_film_list() {
        let mem_film_repo = create_empty_store();

        let films = mem_film_repo.get_films().await;
        let expected = vec![];

        assert!(films.is_ok());
        assert_eq!(films.unwrap(), expected);
    }

    #[actix_rt::test]
    async fn nonempty_store_will_return_a_film_list() {
        let mem_film_repo = create_empty_store();

        let film = CreateFilm {
            title: String::from("Star Wars: The Force Awakens"),
            director: String::from("J. J. Abrams"),
            year: 2015,
            poster: String::from("AWAKEN THE FORCE WITHIN"),
        };

        let result = mem_film_repo.create_film(&film).await;
        assert!(result.is_ok());

        let film = result.unwrap();
        let expected = vec![film];

        let films = mem_film_repo.get_films().await;
        assert!(films.is_ok());
        assert_eq!(films.unwrap(), expected);
    }

    #[actix_rt::test]
    async fn delete_a_film_from_store_will_return_its_id() {
        let mem_film_repo = create_empty_store();

        let film = CreateFilm {
            title: String::from("Star Wars: The Force Awakens"),
            director: String::from("J. J. Abrams"),
            year: 2015,
            poster: String::from("AWAKEN THE FORCE WITHIN"),
        };

        let result = mem_film_repo.create_film(&film).await;
        assert!(result.is_ok());

        let film_id = result.unwrap().id;
        let expected = film_id.to_string();

        let deleted_film_uuid = mem_film_repo.delete_film(&film_id).await;
        assert!(deleted_film_uuid.is_ok());
        assert_eq!(deleted_film_uuid.unwrap().to_string(), expected);

        let films = mem_film_repo.get_films().await;
        assert!(films.is_ok());
        assert_eq!(films.unwrap().len(), 0);
    }

    #[actix_rt::test]
    async fn repo_must_be_empty_on_new() {
        let repo = MemoryFilmRepository::new();
        let result = repo.get_films().await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[actix_rt::test]
    async fn repo_must_be_empty_on_default() {
        let repo = MemoryFilmRepository::default();
        let result = repo.get_films().await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[actix_rt::test]
    async fn get_film_works() {
        let store = RwLock::new(HashMap::new());
        let film = generate_test_film("1");
        store.write().unwrap().insert(film.id, film.clone());

        let repo = MemoryFilmRepository { store };
        let result = repo.get_film(&film.id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), film);
    }

    #[actix_rt::test]
    async fn get_film_fails_if_file_is_not_present() {
        let film_update = generate_test_film("2");

        let repo = MemoryFilmRepository::default();
        let result = repo.update_film(&film_update).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("does not exist"));
    }

    #[actix_rt::test]
    async fn create_film_works() {
        let store = RwLock::new(HashMap::new());
        let create_film = generate_test_create_film("1");

        let repo = MemoryFilmRepository { store };
        let result = repo.create_film(&create_film).await;

        assert!(result.is_ok());
        let created_file = result.unwrap();
        assert_eq!(created_file.title, create_film.title);
        assert_eq!(created_file.director, create_film.director);
        assert_eq!(created_file.poster, create_film.poster);
        assert_eq!(created_file.year, create_film.year);
        assert!(created_file.created_at.is_some());
    }

    #[actix_rt::test]
    async fn update_film_works() {
        let store = RwLock::new(HashMap::new());
        let film = generate_test_film("1");
        store.write().unwrap().insert(film.id, film.clone());

        let mut film_update = film.clone();
        film_update.title = "new-title".to_string();
        film_update.year = 2002;

        let repo = MemoryFilmRepository { store };
        let result = repo.update_film(&film_update).await;

        assert!(result.is_ok());
        let updated_file = result.unwrap();
        assert_eq!(updated_file.id, film.id);
        assert_ne!(updated_file.title, film.title);
        assert_eq!(updated_file.title, film_update.title);
        assert_eq!(updated_file.director, film.director);
        assert_eq!(updated_file.poster, film.poster);
        assert_ne!(updated_file.year, film.year);
        assert_eq!(updated_file.year, film_update.year);
        assert_eq!(updated_file.created_at, film.created_at);
        assert!(updated_file.updated_at.is_some());
        assert!(film.updated_at.is_none());
    }

    #[actix_rt::test]
    async fn update_film_fails_if_file_is_not_present() {
        let store = RwLock::new(HashMap::new());
        let film = generate_test_film("1");
        store.write().unwrap().insert(film.id, film.clone());

        let film_update = generate_test_film("2");

        let repo = MemoryFilmRepository { store };
        let result = repo.update_film(&film_update).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("does not exist"));
    }

    #[actix_rt::test]
    async fn delete_film_works() {
        let store = RwLock::new(HashMap::new());
        let film = generate_test_film("1");
        store.write().unwrap().insert(film.id, film.clone());

        let repo = MemoryFilmRepository { store };
        let result = repo.delete_film(&film.id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), film.id);
    }

    #[actix_rt::test]
    async fn delete_film_does_not_fail_if_film_is_not_present() {
        let repo = MemoryFilmRepository::default();
        let id = uuid::Uuid::new_v4();
        let result = repo.delete_film(&id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), id);
    }
}
