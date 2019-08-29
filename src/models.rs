use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde_derive::{Serialize, Deserialize};
use super::schema::users;
use super::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub first_name: String,
    pub second_name: String,
    pub last_name: String,
    pub birth_date: String
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub user_name: String,
    pub first_name: String,
    pub second_name: String,
    pub last_name: String,
    pub birth_date: String
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
            user_name as us_n,
            first_name as f_n,
            second_name as s_n,
            last_name as l_n,
            birth_date as b_d
        };
        let NewUser { user_name, first_name, second_name, last_name, birth_date } = user;
        diesel::update(all_users.find(id))
            .set((us_n.eq(user_name), f_n.eq(first_name), s_n.eq(second_name), l_n.eq(last_name), b_d.eq(birth_date)))
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
