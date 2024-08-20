use warp::Filter;
use serde_json::json;
use crate::lib_logic::ECO_LEGACY_INSTANCE;
use crate::db::{Project, establish_connection};

#[tokio::main]
async fn main() {
    // Define routes for API endpoints
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

    let create_project = warp::path("projects")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |data: serde_json::Value| {
            let title = data["title"].as_str().unwrap_or("").to_string();
            let description = data["description"].as_str().unwrap_or("").to_string();
            let image_url = data["image_url"].as_str().map(|s| s.to_string());
            let multimedia_url = data["multimedia_url"].as_str().map(|s| s.to_string());
            let instance = ECO_LEGACY_INSTANCE.clone();
            async move {
                let result = instance.lock().await.create_project(&title, &description, image_url.as_deref(), multimedia_url.as_deref()).await;
                Ok::<_, warp::Rejection>(warp::reply::json(&json!({ "result": result })))
            }
        });

    let get_projects = warp::path("projects")
        .and(warp::get())
        .and_then(move || {
            let instance = ECO_LEGACY_INSTANCE.clone();
            async move {
                let result = instance.lock().await.get_projects().await;
                Ok::<_, warp::Rejection>(warp::reply::json(&json!({ "projects": result })))
            }
        });

    let chatbot = warp::path("chatbot")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|data: serde_json::Value| {
            let query = data["query"].as_str().unwrap_or("").to_string();
            async move {
                // Implement chatbot logic or integrate with a chatbot service here
                Ok::<_, warp::Rejection>(warp::reply::json(&json!({ "response": format!("You asked: {}", query) })))
            }
        });

    let swagger_ui = warp::path("swagger")
        .and(warp::get())
        .and(warp::fs::file("swagger-editor/swagger.html"));

    let openapi_json = warp::path("openapi.json")
        .and(warp::get())
        .and(warp::fs::file("openapi.json"));

    // Combine all routes
    let routes = register.or(login).or(create_project).or(get_projects).or(chatbot).or(swagger_ui).or(openapi_json);

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

