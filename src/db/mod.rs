use crate::diesel_migrations::MigrationHarness;
use diesel::r2d2;
use diesel::SqliteConnection;
use diesel_migrations::EmbeddedMigrations;

use crate::errors::{Error, Result};

pub(crate) mod models;
pub(crate) mod schema;

const MIGRATIONS: EmbeddedMigrations = diesel_migrations::embed_migrations!("./migrations");

#[derive(Clone)]
pub struct Pool(pub r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>);

impl Pool {
    pub fn new(database_url: String) -> Result<Self> {
        // create db connection pool
        let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .map_err(Error::internal)?;
        Ok(Pool(pool))
    }

    pub fn get(&self) -> Result<r2d2::PooledConnection<r2d2::ConnectionManager<SqliteConnection>>> {
        self.0.get().map_err(Error::internal)
    }
}

#[tracing::instrument]
pub fn build_pool(database_url: String) -> Result<Pool> {
    // create db connection pool
    let pool = Pool::new(database_url)?;
    let conn = &mut pool.get()?;

    conn.run_pending_migrations(MIGRATIONS)
        .map_err(Error::internal)?;
    Ok(pool)
}
