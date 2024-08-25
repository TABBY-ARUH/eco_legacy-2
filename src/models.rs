use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewProject {
    pub name: String,
    pub description: String,
}
