use serde_json::Value;
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono;
use crate::db::{Project, establish_connection};

pub struct EcoLegacy;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

impl EcoLegacy {
    pub async fn register_user(&self, username: &str, password: &str) -> Result<String, AppError> {
        let hashed_password = hash(password, DEFAULT_COST)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        // Here you should insert the user into the database
        Ok(format!("User {} registered successfully", username))
    }

    pub async fn login_user(&self, username: &str, password: &str) -> Result<String, AppError> {
        let conn = establish_connection();
        // Fetch hashed_password from the database
        let hashed_password = ""; // Fetch from database

        if verify(password, &hashed_password)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
        {
            let claims = Claims {
                sub: username.to_string(),
                exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("secret".as_ref()),
            )
            .map_err(|e| AppError::AuthenticationError(e.to_string()))?;
            Ok(token)
        } else {
            Err(AppError::AuthenticationError(
                "Invalid username or password".into(),
            ))
        }
    }

    pub async fn create_project(&self, title: &str, description: &str, image_url: Option<&str>, multimedia_url: Option<&str>) -> Result<(), AppError> {
        let conn = establish_connection();
        let new_project = Project {
            id: 0, // This will be auto-incremented
            title: title.to_string(),
            description: description.to_string(),
            image_url: image_url.map(|s| s.to_string()),
            multimedia_url: multimedia_url.map(|s| s.to_string()),
        };

        diesel::insert_into(crate::schema::projects::table)
            .values(&new_project)
            .execute(&conn)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, AppError> {
        let conn = establish_connection();
        let results = crate::schema::projects::table
            .load::<Project>(&conn)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(results)
    }
}

lazy_static! {
    pub static ref ECO_LEGACY_INSTANCE: Arc<Mutex<EcoLegacy>> = Arc::new(Mutex::new(EcoLegacy));
}


