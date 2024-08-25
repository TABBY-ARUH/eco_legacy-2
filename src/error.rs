use warp::{Rejection, Reply};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InternalServerError(String),
    NotFound(String),
    BadRequest(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl warp::reject::Reject for AppError {}

pub async fn handle_error(err: Rejection) -> Result<impl Reply, warp::Rejection> {
    if let Some(AppError::InternalServerError(msg)) = err.find() {
        return Ok(warp::reply::with_status(msg, warp::http::StatusCode::INTERNAL_SERVER_ERROR));
    }
    if let Some(AppError::NotFound(msg)) = err.find() {
        return Ok(warp::reply::with_status(msg, warp::http::StatusCode::NOT_FOUND));
    }
    if let Some(AppError::BadRequest(msg)) = err.find() {
        return Ok(warp::reply::with_status(msg, warp::http::StatusCode::BAD_REQUEST));
    }

    Ok(warp::reply::with_status("Unhandled error".to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR))
}
