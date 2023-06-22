use axum::{ routing::{post}, Router, http::Method };
use tower_http::cors::{Any,CorsLayer};

mod auth_handler;

// use crate routes::

pub fn auth_router() -> Router {

    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

    Router::new()
    .route("/register",post(auth_handler::register))
    .route("/login",post(auth_handler::login))
    .layer(cors)
}


