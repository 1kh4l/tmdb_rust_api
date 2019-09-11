use serde_derive;
use reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[derive(Deserialize, Debug)]
struct Movie {
    adult: bool,
    backdrop_path: String,
    genre_ids: Vec<u32>,
    id: u32,
    original_language: String,
    original_title: String,
    overview: String,
    popularity: f32,
    poster_path: String,
    release_date: String,
    title: String,
    video: bool,
    vote_average: f32,
    vote_count: u32,
}

#[derive(Deserialize, Debug)]
struct Response {
    page: u32,
    results: Vec<Movie>,
    total_pages: u32,
    total_results: u32,
}

pub fn api_github() -> Result<(), Error> {
    let tmdb_url = format!("https://api.themoviedb.org/3/movie/{endpoint_popular}?api_key={key}",
                            endpoint_popular = "popular",
                            key = "f9f09a70d4cb6d872c1f8aa8bc859cc3");
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{:?}", request_url);
    println!("{:?}", tmdb_url);
    // let mut response = reqwest::get(&request_url)?;
    let mut response = reqwest::get(&tmdb_url)?;
    let mut response2 = reqwest::get(&request_url)?;
    let users: Vec<User> = response2.json()?;
    println!("{:?}", users);
    let movies: Response = response.json()?;
    println!("{:?}", movies);
    Ok(())
}
