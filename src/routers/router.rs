use axum::{ routing::post, Router };


// use crate routes::

pub fn create_router() -> Router {
    Router::new()
    .route("/api/v1/register",post())
}