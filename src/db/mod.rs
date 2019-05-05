use diesel::{Connection, SqliteConnection};
use diesel_migrations::{find_migrations_directory, run_pending_migrations_in_directory};

pub mod models;
pub mod schema;

pub fn run_migrations(database_url: &str) {
//    if let Ok(migrations_dir) = find_migrations_directory() {
//        if let Ok(connection) = SqliteConnection::establish(database_url) {
//            if let Ok(()) = run_pending_migrations_in_directory(
//                &connection,
//                &migrations_dir,
//                &mut std::io::stdout()) {
//                println!("Migrations Done!");
//            }
//        }
//    }

    match find_migrations_directory() {
        Ok(dir) => {
            match SqliteConnection::establish(database_url) {
                Ok(connection) => {
                    let _migration = run_pending_migrations_in_directory(
                        &connection,
                        &dir,
                        &mut std::io::stdout(),
                    );
                }
                Err(err) => {
                    println!("Error getting Connection {}", err);
                }
            }
        }
        Err(err) => {
            println!("Error getting migration dir{}", err);
        }
    }
}
