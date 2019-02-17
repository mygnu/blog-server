#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;


use std::env;

use actix::prelude::*;
use actix_web::server;
use diesel::{PgConnection, r2d2::ConnectionManager};
use dotenv::dotenv;

use models::DbExecutor;

mod schema;
mod models;
mod app;
mod like_handler;
mod errors;

fn main() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "blog-server=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sys = actix::System::new("blog-server");

    // create db connection pool
//    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("Failed to create pool.");

    let address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    server::new(move || app::create_app(address.clone()))
        .bind("127.0.0.1:3000")
        .expect("Can not bind to '127.0.0.1:3000'")
        .start();

    sys.run();
}
