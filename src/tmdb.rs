use reqwest;
use super::graphql_types::*;
use reqwest::Error;

const TMDB_URL: &str = "https://api.themoviedb.org/3";

// It fetch the popular movies
pub fn fetch_popular_movies() -> Result<MoviesResponse, Error> {
    let tmdb_url = format!("{api}/movie/{endpoint_popular}?api_key={key}",
                            api = &TMDB_URL,
                            endpoint_popular = "popular",
                            key = "f9f09a70d4cb6d872c1f8aa8bc859cc3");
    let mut response = reqwest::get(&tmdb_url)?;
    let movies: MoviesResponse = response.json()?;
    Ok(movies)
}
