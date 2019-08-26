use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde_derive::{Serialize, Deserialize};
use super::schema::users;
use super::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub bio: String,
    pub age: i32,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub bio: String,
    pub age: i32,
    pub image_url: String,
}

impl User {
    // Fetching user.
    pub fn show(id: i32, conn: &PgConnection) -> Vec<User> {
        all_users.find(id)
            .load::<User>(conn)
            .expect("Sometimes users don't show up when you are not an admin")
    }
    // Fetching all users.
    pub fn all(conn: &PgConnection) -> Vec<User> {
        all_users.order(users::id.desc())
            .load::<User>(conn)
            .expect("Error herding users")
    }
    // Creating user.
    pub fn create(user: NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }
    // Updating user.
    pub fn update_by_id(id: i32, user: NewUser, conn: &PgConnection) -> bool {
        use super::schema::users::dsl::{
            name as n,
            bio as b,
            age as k,
            image_url as img,
        };
        let NewUser { name, bio, age, image_url } = user;
        diesel::update(all_users.find(id))
            .set((n.eq(name), b.eq(bio), k.eq(age), img.eq(image_url)))
            .get_result::<User>(conn)
            .is_ok()
    }
    // Deleting user.
    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if User::show(id, conn).is_empty() {
            return false;
        }
        diesel::delete(all_users.find(id))
            .execute(conn)
            .is_ok()
    }
}
