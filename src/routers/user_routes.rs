use axum::{ routing::{post, get }, Router, http::Method };
use tower_http::cors::{Any,CorsLayer};

mod user_handler;
mod comunity_handler;
mod review_handler;
use crate::utils::config;

pub fn user_router() -> Router {

    
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST,Method::PUT])
    .allow_origin(Any);

    Router::new()
    .route(&config::endpoint("/user") ,get(user_handler::user))
    .route(&config::endpoint("/user"),post(user_handler::update))
    .route(&config::endpoint("/user/tech"),post(user_handler::add_tech))
    .route(&config::endpoint("/user/tech"),get(user_handler::user_tech))
    .route(&config::endpoint("/user/reviews"),get(review_handler::get_review))
    .route(&config::endpoint("/user/slots/caption/:caption_id"),get(review_handler::get_caption_slots))
    .route(&config::endpoint("/user/slot/:uuid"),get(review_handler::get_slot))
    .route(&config::endpoint("/user/slot/:uuid/book"),get(review_handler::book_slot))
    .route(&config::endpoint("/user/slot/create"),post(review_handler::create_slot))
    .route(&config::endpoint("/user/slot/:uuid/rate"),post(review_handler::save_review))
    // .route("/user/community",post(comunity_handler::create))
    // .route("/user/community/:uuid",put(comunity_handler::update))
    // .route("/login",post(auth_handler::login))
    .layer(cors)
}


