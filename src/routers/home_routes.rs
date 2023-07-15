use axum::{ routing::get , Router, http::Method };
use tower_http::cors::{Any,CorsLayer};

mod home_handler;
use crate::utils::config;

pub fn home_routes() -> Router {
    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET])
    .allow_origin(Any);

    Router::new()
    .route(&config::endpoint("/home/tech"),get(home_handler::get_tech))
    .layer(cors)
}


