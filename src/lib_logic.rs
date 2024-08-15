use serde_json::Value;
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct EcoLegacy;

impl EcoLegacy {
    pub async fn register_user(&self, username: &str, _password: &str) -> Result<String, String> {
        // Add logic for user registration
        Ok(format!("User {} registered successfully", username))
    }

    pub async fn login_user(&self, username: &str, _password: &str) -> Result<String, String> {
        // Add logic for user login
        Ok(format!("User {} logged in successfully", username))
    }
}

lazy_static! {
    pub static ref ECO_LEGACY_INSTANCE: Arc<Mutex<EcoLegacy>> = Arc::new(Mutex::new(EcoLegacy));
}


