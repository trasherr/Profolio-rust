use axum::{ routing::{post, get, put}, Router, http::Method };
use tower_http::cors::{Any,CorsLayer};

mod user_handler;
mod comunity_handler;

// use crate routes::

pub fn user_router() -> Router {

    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST,Method::PUT])
    .allow_origin(Any);

    Router::new()
    .route("/user",get(user_handler::user))
    .route("/user",post(user_handler::update))
    .route("/user/tech",post(user_handler::add_tech))
    .route("/user/tech",get(user_handler::user_tech))
    // .route("/user/community",post(comunity_handler::create))
    // .route("/user/community/:uuid",put(comunity_handler::update))
    // .route("/login",post(auth_handler::login))
    .layer(cors)
}


