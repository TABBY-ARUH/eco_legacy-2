// tests/integration_tests.rs

#[tokio::test]
async fn test_register_api() {
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3030/register")
        .json(&serde_json::json!({
            "username": "testuser",
            "password": "password"
        }))
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());
}

#[tokio::test]
async fn test_login_api() {
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3030/login")
        .json(&serde_json::json!({
            "username": "testuser",
            "password": "password"
        }))
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());
}
