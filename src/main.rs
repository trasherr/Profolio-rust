use std::env;
use dotenv::dotenv;

#[macro_use]
extern crate rbatis;
extern crate rbs;

#[path = "./models/users.rs"]
mod users;

#[path = "./index.rs"]
mod index;


#[tokio::main]
async fn main() {

    dotenv().ok();
    use rbatis::Rbatis;
    use rbdc_pg::driver::PgDriver;

    // println!("{}", env::var("DATABASE_URL").expect("No Connection"));

    let conn = env::var("DATABASE_URL").expect("No Connection");

    let rb = Rbatis::new();
    rb.init(PgDriver {}, &conn).unwrap();

    // users::drop(&rb).await.map_err(|err| println!("{:?}", err)).ok();
    users::up(&rb).await;


    index::server();

}

