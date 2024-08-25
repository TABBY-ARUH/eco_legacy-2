#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use crate::lib_logic::EcoLegacy;

    #[tokio::test]
    async fn test_register_user() {
        let eco_legacy_instance = Arc::new(Mutex::new(EcoLegacy::new()));
        let result = eco_legacy_instance.lock().await.register_user("testuser", "testpassword").await;
        assert!(result.is_ok()); // Assuming register_user returns Result
    }

    #[tokio::test]
    async fn test_login_user() {
        let eco_legacy_instance = Arc::new(Mutex::new(EcoLegacy::new()));
        eco_legacy_instance.lock().await.register_user("testuser", "testpassword").await.unwrap();
        let result = eco_legacy_instance.lock().await.login_user("testuser", "testpassword").await;
        assert!(result.is_ok()); // Assuming login_user returns Result
    }
}
