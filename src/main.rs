#[macro_use]
extern crate diesel;

use actix::prelude::*;
use actix_web::server;
use config::ConfigError;
use diesel::{r2d2::ConnectionManager, SqliteConnection};

use db::models::DbExecutor;

mod db;
mod app;
mod settings;

fn main() -> Result<(), ConfigError> {
    std::env::set_var("RUST_LOG", "blog-server=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let settings = crate::settings::Settings::new()?;

    db::run_migrations(settings.database_url.as_ref());

//    let migrations_dir = migrations::find_migrations_directory().unwrap();
//    migrations::run_pending_migrations_in_directory(
//        &SqliteConnection::establish(&settings.database_url).unwrap(),
//        &migrations_dir,
//        &mut io::stdout(),
//    ).unwrap();


    let sys = actix::System::new("blog-server");

    // create db connection pool
    let pool = r2d2::Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(settings.database_url))
        .expect("Failed to create pool.");
    let address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    server::new(move || app::create_app(address.clone()))
        .bind(settings.server_url.as_str())
        .expect(format!("Can not bind to '{}'", settings.server_url).as_str())
        .run();

    sys.run();
    Ok(())
}
