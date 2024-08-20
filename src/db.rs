use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "projects"]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub multimedia_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
}

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("eco_legacy.db").expect("Error connecting to the database")
}
