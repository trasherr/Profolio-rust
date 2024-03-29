use utils::guard;
use axum::{Router, Extension, middleware};
use sea_orm::Database;

// use tokio_cron_scheduler::{JobScheduler, Job};

mod routers;
mod utils;
mod models;

#[tokio::main]
async fn main() {
    
    //+++++++++++++++++++++++++++++++++++++++

    // let sched = JobScheduler::new().await.unwrap();
  
    // sched.add(Job::new_async("0 0 1 * * Sun",  |_uuid, _l| Box::pin(async{

    //     println!("Cron Each Sunday 1 am");

    //     let conn_str = env::var("DATABASE_URL").expect("No Connection");
    //     let conn1 = Database::connect(conn_str).await.expect("Database connection failed");
    //     utils::crons::create_leaderboard(&conn1).await;

    //     conn1.close().await.unwrap();
    // })).unwrap()).await.unwrap();
   
    // sched.start().await.unwrap();
    //+++++++++++++++++++++++++++++++++++++++++++++++
    
    server().await;

    // utils::crons::create_leaderboard(conn)

}

pub async fn server() {
   
    let conn_str = (*utils::constants::DATABASE_URL).clone();
    let conn = Database::connect(conn_str).await.expect("Database connection failed");

    let app = Router::new()

    .merge(routers::user_routes::user_router())
    .route_layer(middleware::from_fn(guard::user_guard))
    .merge(routers::auth_routes::auth_router())
    .merge(routers::home_routes::home_routes())
    .merge(routers::admin_routes::web_router())
    .layer(Extension(conn));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

