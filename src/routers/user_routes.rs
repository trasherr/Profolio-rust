use axum::{ routing::{post, get }, Router, http::Method };
use tower_http::cors::{Any,CorsLayer};

mod user_handler;
mod comunity_handler;
mod review_handler;
mod roadmap_handler;
mod order_handler;
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
    .route(&config::endpoint("/user/targets"),post(user_handler::get_target_post))
    .route(&config::endpoint("/user/caption/apply"),get(user_handler::get_apply_caption))

    //roadmap routes
    .route(&config::endpoint("/user/roadmap"),post(roadmap_handler::roadmap_post))
    .route(&config::endpoint("/user/roadmap"),get(roadmap_handler::roadmap_get))

    //slots and review routes
    .route(&config::endpoint("/user/meetings"),get(review_handler::get_review))
    .route(&config::endpoint("/user/meetings/:meeting_count"),get(review_handler::get_review_count))
    .route(&config::endpoint("/user/slots/caption/:caption_id"),get(review_handler::get_caption_slots))
    .route(&config::endpoint("/user/slot/:slot_uuid"),get(review_handler::get_slot))
    .route(&config::endpoint("/user/slot/book/:order_id"),get(review_handler::book_slot))
    .route(&config::endpoint("/user/slot/create"),post(review_handler::create_slot))
    .route(&config::endpoint("/user/slot/:slot_uuid/rate"),post(review_handler::save_review))

    //order routes
    .route(&config::endpoint("/user/order/slot/:slot_uuid"),post(order_handler::post_order))
    .route(&config::endpoint("/user/order/signature"),post(order_handler::post_order_signature))
    
    .layer(cors)
}


