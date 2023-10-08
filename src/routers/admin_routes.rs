use axum::{ routing::get, Router, http::Method };
use tower_http::cors::{Any,CorsLayer};

mod home_handler;

use crate::utils::config;

pub fn web_router() -> Router {

    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST,Method::PUT])
    .allow_origin(Any);

    Router::new()
    .route(&config::endpoint("/web/delete_user/:email") ,get(home_handler::em_user_get))
    .layer(cors)
}


