use axum::{ routing::{post, get}, Router, http::Method };
use tower_http::cors::{Any,CorsLayer};

mod auth_handler;
use crate::utils::config;

pub fn auth_router() -> Router {

    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin(Any);

    Router::new()
    .route(&config::endpoint("/auth/register"),post(auth_handler::register))
    .route(&config::endpoint("/auth/login"),post(auth_handler::login))
    .route(&config::endpoint("/auth/forget/:user_email"),get(auth_handler::forget_password))
    .route(&config::endpoint("/auth/forget/otp"),post(auth_handler::check_otp))
    .route(&config::endpoint("/auth/password/change"),post(auth_handler::change_password))
    .layer(cors)
}


