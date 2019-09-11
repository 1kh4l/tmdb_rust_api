use reqwest;
use super::graphql_types::*;
use reqwest::Error;
//
pub fn fetch_all_movies() -> Result<MoviesResponse, Error> {
    let tmdb_url = format!("https://api.themoviedb.org/3/movie/{endpoint_popular}?api_key={key}",
                            endpoint_popular = "popular",
                            key = "f9f09a70d4cb6d872c1f8aa8bc859cc3");
    println!("{:?}", tmdb_url);
    let mut response = reqwest::get(&tmdb_url)?;
    let movies: MoviesResponse = response.json()?;
    println!("{:?}", movies);
    Ok(movies)
}
