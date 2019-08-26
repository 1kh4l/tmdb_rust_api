#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate rocket_cors;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};

use routes::*;

mod db;
mod models;
mod routes;
mod schema;

fn rocket() -> rocket::Rocket {
    let pool = db::create_db_pool();
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);
    let options = rocket_cors::CorsOptions {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Put, Method::Post, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    };
    rocket::ignite()
        .manage(pool)
        .mount("/api", routes![all_users, new_user, update_user, delete_user])
        .mount("/", routes![index])
        .manage(options)
}

#[get("/")]
fn index<'a>() -> &'a str {
    "Hello!"
}

fn main() {
    rocket().launch();
}
