use warp::{Filter, Rejection, Reply};
use serde_json::json;
use crate::db; // Import the db module
use crate::error::AppError; // Import the custom error types

pub async fn register_user() -> Result<impl Reply, Rejection> {
    match db::register_user().await {
        Ok(_) => Ok(warp::reply::json(&json!({"status": "User registered"}))),
        Err(e) => Err(warp::reject::custom(AppError::InternalServerError(format!("Error registering user: {:?}", e)))),
    }
}

pub async fn login_user() -> Result<impl Reply, Rejection> {
    match db::login_user().await {
        Ok(_) => Ok(warp::reply::json(&json!({"status": "User logged in"}))),
        Err(e) => Err(warp::reject::custom(AppError::InternalServerError(format!("Error logging in user: {:?}", e)))),
    }
}

pub async fn get_all_projects() -> Result<impl Reply, Rejection> {
    match db::get_all_projects().await {
        Ok(projects) => Ok(warp::reply::json(&projects)),
        Err(e) => Err(warp::reject::custom(AppError::InternalServerError(format!("Error fetching projects: {:?}", e)))),
    }
}

pub async fn add_new_project(new_project: crate::models::NewProject) -> Result<impl Reply, Rejection> {
    match db::add_new_project(new_project).await {
        Ok(_) => Ok(warp::reply::json(&json!({"status": "Project added"}))),
        Err(e) => Err(warp::reject::custom(AppError::InternalServerError(format!("Error adding project: {:?}", e)))),
    }
}

pub async fn get_project_by_id(id: u32) -> Result<impl Reply, Rejection> {
    match db::get_project_by_id(id).await {
        Ok(project) => Ok(warp::reply::json(&project)),
        Err(e) => Err(warp::reject::custom(AppError::InternalServerError(format!("Error fetching project: {:?}", e)))),
    }
}

pub async fn update_project(id: u32, updated_project: crate::models::Project) -> Result<impl Reply, Rejection> {
    match db::update_project(id, updated_project).await {
        Ok(_) => Ok(warp::reply::json(&json!({"status": "Project updated"}))),
        Err(e) => Err(warp::reject::custom(AppError::InternalServerError(format!("Error updating project: {:?}", e)))),
    }
}

pub async fn delete_project(id: u32) -> Result<impl Reply, Rejection> {
    match db::delete_project(id).await {
        Ok(_) => Ok(warp::reply::json(&json!({"status": "Project deleted"}))),
        Err(e) => Err(warp::reject::custom(AppError::InternalServerError(format!("Error deleting project: {:?}", e)))),
    }
}

pub fn api_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let register = warp::path("api").and(warp::path("auth").and(warp::post()).and_then(register_user));
    let login = warp::path("api").and(warp::path("auth").and(warp::post()).and_then(login_user));
    let get_projects = warp::path("api").and(warp::path("projects").and(warp::get()).and_then(get_all_projects));
    let add_project = warp::path("api").and(warp::path("projects").and(warp::post()).and_then(add_new_project));
    let get_project = warp::path("api").and(warp::path("projects").and(warp::path::param()).and(warp::get()).and_then(get_project_by_id));
    let update_project = warp::path("api").and(warp::path("projects").and(warp::path::param()).and(warp::put()).and_then(update_project));
    let delete_project = warp::path("api").and(warp::path("projects").and(warp::path::param()).and(warp::delete()).and_then(delete_project));

    register
        .or(login)
        .or(get_projects)
        .or(add_project)
        .or(get_project)
        .or(update_project)
        .or(delete_project)
        .recover(crate::error::handle_error) // Use the centralized error handler
}


