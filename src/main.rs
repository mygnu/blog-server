extern crate diesel;
extern crate diesel_migrations;

use std::net::{Ipv4Addr, SocketAddr};

use actix_web::{middleware, web::Data, App, HttpServer};

use tracing::Level;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

use app::posts::{get_likes, like_post};

mod app;
mod db;
pub(crate) mod errors;

use crate::errors::Result;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the database file
    #[arg(short, long, default_value = "/tmp/blog.db")]
    db_path: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) pool: db::Pool,
}

#[actix_web::main]
#[tracing::instrument]
async fn main() -> Result<()> {
    let args = Args::parse();
    std::env::set_var(
        "RUST_LOG",
        "blog-server=debug,actix_web=info,actix_server=info",
    );
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_BACKTRACE", "1");

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(Level::ERROR.into())
                .from_env_lossy(),
        )
        .init();

    tracing::trace!("Tracing initialized");

    // create db connection pool
    let pool = db::build_pool(args.db_path)?;
    let socket_address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, args.port));

    tracing::info!("Starting server at: {}", socket_address);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { pool: pool.clone() }))
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(like_post)
            .service(get_likes)
    })
    .bind(socket_address)?
    .run()
    .await
    .map_err(Into::into)
}
