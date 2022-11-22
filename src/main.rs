use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::{Serialize};
use warp::{Filter, Rejection};
use warp::header::headers_cloned;
use warp::http::HeaderMap;
use warp::reject::Reject;
use warp::reply::Json;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserResponse {
    user_id: u64,
    name: String
}

#[derive(Debug)]
pub enum Error {
//    #[error("error")]
    SomeError(),
    // #[error("no authorization header found")]
    NoAuthHeaderFoundError,
    // #[error("wrong authorization header format")]
    InvalidAuthHeaderFormatError,
    // #[error("no user found for this token")]
    InvalidTokenError,
    // #[error("error during authorization")]
    AuthorizationError,
    // #[error("user is not unauthorized")]
    UnauthorizedError,
    // #[error("no user found with this name")]
    UserNotFoundError,
}

impl Reject for Error {}

#[tokio::main]
async fn main() {
    let user_info = warp::path!("user" / "info.json")
        .and(with_authentication())
        .map(handle_user_info)
        .with(warp::reply::with::header("Access-Control-Allow-Origin", "*"));

    warp::serve(user_info)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn handle_user_info(auth_token: String) -> Json {
    let user_id = calculate_hash(&auth_token);
    let name = format!("{}_name", &auth_token);
    warp::reply::json(&UserResponse { user_id, name })
}

fn with_authentication() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .and_then(authenticate)
}

async fn authenticate(headers: HeaderMap) -> Result<String, Rejection> {
    let authorization_header = headers.get("authorization")
        .ok_or(warp::reject::custom(Error::NoAuthHeaderFoundError))?;

    let authorization_header = authorization_header.to_str()
        .map_err(|_| warp::reject::custom(Error::NoAuthHeaderFoundError))?;

    let auth_token = authorization_header.strip_prefix("Bearer ")
        .ok_or(warp::reject::custom(Error::NoAuthHeaderFoundError))?;

    Ok(auth_token.to_string())
}

fn calculate_hash(string: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}