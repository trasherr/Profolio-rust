use axum::{ routing::{post, get}, Router };


mod auth_handler;

// use crate routes::

pub fn auth_router() -> Router {
    Router::new()
    .route("/register",post(auth_handler::register))
    .route("/login",post(auth_handler::login))
}


