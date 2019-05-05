use actix::Addr;
use actix_web::{App, http::Method, middleware::Logger};

use crate::db::models::{AppState, DbExecutor};

use self::like_handler::{add_like, get_post};

mod errors;
mod like_handler;


/// creates and returns the app after mounting all routes/resources
pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {
    App::with_state(AppState { db })
        .middleware(Logger::default())
        .resource("/posts/{id}", |r| {
            r.method(Method::GET).with(get_post);
            r.method(Method::POST).with(add_like);
        })
}
