extern crate dotenv;

use dotenv::dotenv;
use utils::guard;
use std::env;

use axum::{Router, Extension, middleware};


// pub use self::error::{Error, Result};
// use crate::ctx::Ctx;
// use crate::log::log_request;
// use crate::model::ModelController;
// use axum::extract::{Path, Query};
// use axum::http::{Method, Uri};
// use axum::response::{Html, IntoResponse, Response};
// use axum::routing::{get, get_service};
// use serde::Deserialize;
// use serde_json::json;
// use std::net::SocketAddr;
// use tower_cookies::CookieManagerLayer;
// // use tower_http::services::ServeDir;
// use uuid::Uuid;
use sea_orm::{Database, DatabaseConnection};

mod routers;
mod utils;
mod models;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let conn_str = env::var("DATABASE_URL").expect("No Connection");
    let conn = Database::connect(conn_str).await.expect("Database connection failed");

    server(conn).await;

}

pub async fn server(conn: DatabaseConnection) {
    
    let app = Router::new()
    .merge(routers::user_routes::user_router())
    .route_layer(middleware::from_fn(guard::guard))
    .merge(routers::auth_routes::auth_router())
    .layer(Extension(conn));
  
    
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

