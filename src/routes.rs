use rocket_contrib::json::{Json, JsonValue};
use super::db::DbConn;
use super::models::{User, NewUser};

#[get("/users")]
pub fn all_users(conn:DbConn) -> JsonValue {
    let users = User::all(&conn);
    json!({
        "status": 200,
        "result": users,
    })
}

#[post("/users", data = "<new_user>")]
pub fn new_user(new_user: Json<NewUser>, conn: DbConn) -> JsonValue {
    json!({
        "status": User::create(new_user.into_inner(), &conn),
        "result": User::all(&conn),
    })
}

#[put("/users/<id>", data = "<new_user>")]
pub fn update_user(id: i32, new_user: Json<NewUser>, conn: DbConn) -> JsonValue {
    let status = if User::update_by_id(id, new_user.into_inner(), &conn) { 200 } else { 404 };
    json!({
        "status": status,
        "result": User::all(&conn),
    })
}

#[delete("/users/<id>")]
pub fn delete_user(id: i32, conn: DbConn) -> JsonValue {
    let status = if User::delete_by_id(id, &conn) { 200 } else { 404 };
    json!({
        "status": status,
        "result": null,
    })
}
