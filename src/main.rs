#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix::prelude::*;
use actix_web::{middleware::Logger, web, App, HttpServer};
use config::ConfigError;
use diesel::{r2d2::ConnectionManager, Connection, SqliteConnection};

use app::like_handler::{add_like, get_post};
use db::models::{AppData, DbExecutor};

mod app;
mod db;
mod settings;

embed_migrations!("migrations");

fn main() -> Result<(), ConfigError> {
    std::env::set_var(
        "RUST_LOG",
        "blog-server=debug,actix_web=info,actix_server=info",
    );
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();
    let settings = crate::settings::Settings::new()?;
    let cpu_cores = num_cpus::get();

    match SqliteConnection::establish(settings.database_url.as_ref()) {
        Ok(connection) => {
            let _ = embedded_migrations::run_with_output(&connection, &mut std::io::stdout());
        }
        Err(err) => {
            println!("Error getting Connection {}", err);
        }
    }
    let sys = actix::System::new("blog-server");

    // create db connection pool
    let pool = r2d2::Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(
            settings.database_url,
        ))
        .expect("Failed to create pool.");
    let address: Addr<DbExecutor> = SyncArbiter::start(cpu_cores, move || DbExecutor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .data(AppData {
                db: address.clone(),
            })
            .wrap(Logger::default())
            .service(
                web::resource("/api/posts/{id}")
                    .route(web::get().to_async(get_post))
                    .route(web::post().to_async(add_like)),
            )
    })
    .bind(settings.server_url.as_str())
    .expect("Can not bind to port")
    .start();

    let _ = sys.run();
    Ok(())
}
