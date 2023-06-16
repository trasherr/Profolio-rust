extern crate dotenv;

use dotenv::dotenv;
use std::env;

use axum::Router;


// pub use self::error::{Error, Result};
// use crate::ctx::Ctx;
// use crate::log::log_request;
// use crate::model::ModelController;
use axum::extract::{Path, Query};
use axum::http::{Method, Uri};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
// use tower_http::services::ServeDir;
use uuid::Uuid;
use sea_orm::{Database, DatabaseConnection};

mod routers;


#[tokio::main]
async fn main() {

    dotenv().ok();

    let conn_str = env::var("DATABASE_URL").expect("No Connection");
    let _conn = Database::connect(conn_str).await.expect("Database connection failed");
    server().await;

}

pub async fn server() {
    
    let app = Router::new().merge(routers::auth_routes::auth_router());
    //.route("/", get(|| async { "Hello, World!" }));

    
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

