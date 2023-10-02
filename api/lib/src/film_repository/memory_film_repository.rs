use super::{FilmRepository, FilmResult};
use shared::models::{CreateFilm, Film};
use std::collections::HashMap;

pub struct MemoryFilmRepository {
    store: HashMap<uuid::Uuid, Film>,
}

impl MemoryFilmRepository {
    pub fn new() -> MemoryFilmRepository {
        Self {
            store: HashMap::new(),
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
        Ok(self.store.iter().map(|(_, v)| v.clone()).collect())
    }

    async fn get_film(&self, film_id: &uuid::Uuid) -> FilmResult<Film> {
        match self.store.get(&film_id) {
            Some(v) => Ok(v.clone()),
            None => Err(format!("Film id: {} not found", film_id)),
        }
    }

    async fn create_film(&mut self, create_film: &CreateFilm) -> FilmResult<Film> {
        let id = uuid::Uuid::new_v4();
        let utc_now = chrono::Utc::now();
        let new_film = Film {
            id,
            title: create_film.title.clone(),
            director: create_film.director.clone(),
            year: create_film.year,
            poster: create_film.poster.clone(),
            created_at: Some(utc_now),
            updated_at: Some(utc_now),
        };
        self.store.insert(id, new_film.clone());
        Ok(new_film)
    }

    async fn update_film(&mut self, film: &Film) -> FilmResult<Film> {
        let utc_now = chrono::Utc::now();
        if let Some(the_film) = self.store.get_mut(&film.id) {
            the_film.title = film.title.clone();
            the_film.director = film.director.clone();
            the_film.year = film.year;
            the_film.poster = film.poster.clone();
            the_film.updated_at = Some(utc_now);
            Ok(the_film.clone())
        } else {
            Err(format!("Film id: {} not found", film.id))
        }
    }

    async fn delete_film(&mut self, film_id: &uuid::Uuid) -> FilmResult<uuid::Uuid> {
        self.store
            .remove(&film_id)
            .map(|v| v.id)
            .ok_or(format!("Film id: {} not found", film_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_empty_store() -> MemoryFilmRepository {
        MemoryFilmRepository::new()
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
        let mut mem_film_repo = create_empty_store();

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
        let mut mem_film_repo = create_empty_store();

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
}
