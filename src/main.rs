
use warp::Filter;
use eco_legacy::lib_logic::ECO_LEGACY_INSTANCE;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Define routes
    let register = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |data: serde_json::Value| {
            let username = data["username"].as_str().unwrap_or("").to_string();
            let password = data["password"].as_str().unwrap_or("").to_string();
            let instance = ECO_LEGACY_INSTANCE.clone();
            async move {
                let result = instance.lock().await.register_user(&username, &password).await;
                Ok::<_, warp::Rejection>(warp::reply::json(&json!({ "result": result })))
            }
        });

    let login = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |data: serde_json::Value| {
            let username = data["username"].as_str().unwrap_or("").to_string();
            let password = data["password"].as_str().unwrap_or("").to_string();
            let instance = ECO_LEGACY_INSTANCE.clone();
            async move {
                let result = instance.lock().await.login_user(&username, &password).await;
                Ok::<_, warp::Rejection>(warp::reply::json(&json!({ "result": result })))
            }
        });

    // Combine routes
    let routes = register.or(login);

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
