use warp::{Filter, Rejection, Reply};
use ic_cdk::export::Principal;
use reqwest::Client;
use dotenv::dotenv;
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use std::sync::Arc;
use serde_json::json; // Import the json! macro

#[derive(Debug)]
struct EcoLegacy {
    // Define your struct fields here
}

impl EcoLegacy {
    async fn register_user(&self, _username: &str, _password: &str, _principal_id: Principal) -> String {
        // Your implementation here
        "User registered".to_string()
    }

    async fn login_user(&self, _username: &str, _password: &str) -> Principal {
        // Your implementation here
        Principal::anonymous()
    }

    async fn create_proposal(&self, _title: &str, _description: &str, _creator: Principal) -> u64 {
        // Your implementation here
        1
    }

    async fn vote_on_proposal(&self, _proposal_id: u64, _vote_for: bool) -> bool {
        // Your implementation here
        true
    }
}

lazy_static! {
    static ref ECO_LEGACY_INSTANCE: Arc<Mutex<EcoLegacy>> = Arc::new(Mutex::new(EcoLegacy {
        // Initialize your EcoLegacy instance here
    }));
}

#[tokio::main]
async fn main() {
    // Initialize dotenv for environment variable management
    dotenv().ok();

    let register = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_register);

    let login = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_login);

    let create_proposal = warp::path("create_proposal")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_create_proposal);

    let vote_proposal = warp::path("vote_proposal")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_vote_proposal);

    let chatbot = warp::path("api")
        .and(warp::path("chatbot"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_chatbot_query);

    let translate = warp::path("translate")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_translation);

    let routes = register
        .or(login)
        .or(create_proposal)
        .or(vote_proposal)
        .or(chatbot)
        .or(translate)
        .recover(error::handle_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_register(data: serde_json::Value) -> Result<impl Reply, Rejection> {
    let username = data["username"].as_str().unwrap_or_default().to_string();
    let password = data["password"].as_str().unwrap_or_default().to_string();
    let principal_id = match Principal::from_text(data["principal_id"].as_str().unwrap_or_default()) {
        Ok(id) => id,
        Err(_) => return Ok(warp::reply::with_status(
            warp::reply::json(&json!({"error": "Invalid principal_id"})),
            warp::http::StatusCode::BAD_REQUEST
        )),
    };
    
    // Lock the instance for the operation
    let instance = ECO_LEGACY_INSTANCE.lock().await;
    let result = instance.register_user(&username, &password, principal_id).await;
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "result": result })),
        warp::http::StatusCode::OK,
    ))
}

async fn handle_login(data: serde_json::Value) -> Result<impl Reply, Rejection> {
    let username = data["username"].as_str().unwrap_or_default().to_string();
    let password = data["password"].as_str().unwrap_or_default().to_string();
    
    // Lock the instance for the operation
    let instance = ECO_LEGACY_INSTANCE.lock().await;
    let principal_id = instance.login_user(&username, &password).await;
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "principal_id": principal_id })),
        warp::http::StatusCode::OK,
    ))
}

async fn handle_create_proposal(data: serde_json::Value) -> Result<impl Reply, Rejection> {
    let title = data["title"].as_str().unwrap_or_default().to_string();
    let description = data["description"].as_str().unwrap_or_default().to_string();
    let principal_id = match Principal::from_text(data["creator"].as_str().unwrap_or_default()) {
        Ok(id) => id,
        Err(_) => return Ok(warp::reply::with_status(
            warp::reply::json(&json!({"error": "Invalid principal_id"})),
            warp::http::StatusCode::BAD_REQUEST
        )),
    };
    
    // Lock the instance for the operation
    let instance = ECO_LEGACY_INSTANCE.lock().await;
    let proposal_id = instance.create_proposal(&title, &description, principal_id).await;
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "proposal_id": proposal_id })),
        warp::http::StatusCode::OK,
    ))
}

async fn handle_vote_proposal(data: serde_json::Value) -> Result<impl Reply, Rejection> {
    let proposal_id = data["proposal_id"].as_u64().unwrap_or(0);
    let vote_for = data["vote_for"].as_bool().unwrap_or(true);
    
    // Lock the instance for the operation
    let instance = ECO_LEGACY_INSTANCE.lock().await;
    let result = instance.vote_on_proposal(proposal_id, vote_for).await;
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "result": result })),
        warp::http::StatusCode::OK,
    ))
}

async fn handle_chatbot_query(data: serde_json::Value) -> Result<impl Reply, Rejection> {
    let user_query = data["query"].as_str().unwrap_or_default().to_string();
    let chatbot_response = process_query(user_query).await;
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "response": chatbot_response })),
        warp::http::StatusCode::OK,
    ))
}

async fn handle_translation(data: serde_json::Value) -> Result<impl Reply, Rejection> {
    let text = data["text"].as_str().unwrap_or_default().to_string();
    let target_language = data["target_language"].as_str().unwrap_or_default().to_string();
    let translated_text = perform_translation(text, target_language).await
        .map_err(|_| warp::reject::custom(error::CustomError))?;
    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "translated_text": translated_text })),
        warp::http::StatusCode::OK,
    ))
}

async fn process_query(query: String) -> String {
    format!("You asked: {}", query)
}

async fn perform_translation(text: String, target_language: String) -> Result<String, warp::Rejection> {
    let client = Client::new();
    let url = "https://libretranslate.de/translate";

    let response = client.post(url)
        .json(&json!({
            "q": text,
            "target": target_language,
            "source": "auto" // Set source language if needed
        }))
        .send()
        .await
        .map_err(|_| warp::reject::custom(error::CustomError))?;

    let body = response.json::<serde_json::Value>().await
        .map_err(|_| warp::reject::custom(error::CustomError))?;

    Ok(body["translatedText"].as_str().unwrap_or_default().to_string())
}

mod error {
    use warp::Rejection;

    #[derive(Debug)]
    pub struct CustomError;

    impl warp::reject::Reject for CustomError {}

    pub async fn handle_error(err: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
        if let Some(CustomError) = err.find::<CustomError>() {
            Ok(warp::reply::with_status(
                "Custom error occurred",
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        } else {
            Err(err)
        }
    }
}
