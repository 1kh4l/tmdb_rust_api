use super::models::MyUser;
use super::schema::myusers;

#[derive(Insertable, GraphQLInputObject)]
#[table_name = "myusers"]
pub struct NewUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(AsChangeset)]
// #[changeset_options(treat_none_as_null="true")]
#[table_name = "myusers"]
pub struct UpdateUser {
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(GraphQLObject, Debug)]
pub struct TokenResponse {
    pub token: Option<String>,
}

#[derive(GraphQLObject, Debug)]
pub struct UsersResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub users: Option<Vec<MyUser>>,
}

#[derive(GraphQLObject, Debug)]
pub struct UserResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub user: Option<MyUser>,
}

#[derive(Deserialize, GraphQLObject, Debug)]
pub struct MoviesResponse {
    pub page: i32,
    pub results: Option<Vec<Movie>>,
    pub total_pages: i32,
    pub total_results: i32,
}

#[derive(Deserialize, GraphQLObject, Debug)]
pub struct Movie {
    pub adult: bool,
    pub backdrop_path: String,
    pub genre_ids: Vec<i32>,
    pub id: i32,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: String,
    pub release_date: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i32,
}
