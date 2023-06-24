use axum::{ routing::{post, get}, Router, http::Method };
use tower_http::cors::{Any,CorsLayer};

mod user_handler;

// use crate routes::

pub fn user_router() -> Router {

    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

    Router::new()
    .route("/user",get(user_handler::get_user))
    // .route("/login",post(auth_handler::login))
    .layer(cors)
}


