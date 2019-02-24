#[macro_use]
extern crate diesel;

use actix::prelude::*;
use actix_web::server;
use config::ConfigError;
use diesel::{PgConnection, r2d2::ConnectionManager};

use db::models::DbExecutor;

mod db;
mod app;
mod settings;

fn main() -> Result<(), ConfigError> {
    std::env::set_var("RUST_LOG", "blog-server=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let settings = crate::settings::Settings::new()?;
//    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sys = actix::System::new("blog-server");

    // create db connection pool
//    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(settings.database_url))
        .expect("Failed to create pool.");

    let address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    server::new(move || app::create_app(address.clone()))
        .bind(settings.server_url.as_str())
        .expect(format!("Can not bind to '{}'", settings.server_url).as_str())
        .run();

    sys.run();
    Ok(())
}
