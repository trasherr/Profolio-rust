extern crate dotenv;

use dotenv::dotenv;
use tokio_cron_scheduler::{JobScheduler, Job};
use utils::guard;
use std::env;

use axum::{Router, Extension, middleware};
use sea_orm::{Database, DatabaseConnection};


mod routers;
mod utils;
mod models;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let conn_str = env::var("DATABASE_URL").expect("No Connection");
    let conn = Database::connect(conn_str).await.expect("Database connection failed");


    //+++++++++++++++++++++++++++++++++++++++

    let sched = JobScheduler::new().await.unwrap();
  
    sched.add(Job::new_async("0 0 1 * * Sun",  |_uuid, _l| Box::pin(async{

        println!("Cron Each Sunday 1 am");

        let conn_str = env::var("DATABASE_URL").expect("No Connection");
        let conn1 = Database::connect(conn_str).await.expect("Database connection failed");
        utils::crons::create_leaderboard(&conn1).await;

        conn1.close().await.unwrap();
    })).unwrap()).await.unwrap();
   
    sched.start().await.unwrap();
    //+++++++++++++++++++++++++++++++++++++++++++++++
    
    server(conn).await;

    // utils::crons::create_leaderboard(conn)

}

pub async fn server(conn: DatabaseConnection) {

    let app = Router::new()
    .merge(routers::user_routes::user_router())
    .route_layer(middleware::from_fn(guard::guard))
    .merge(routers::auth_routes::auth_router())

    .merge(routers::home_routes::home_routes())
    .layer(Extension(conn));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

