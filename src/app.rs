use actix::Addr;
use actix_web::{App, http::Method, middleware::Logger};

use crate::like_handler::{add_like, get_likes};
use crate::models::{AppState, DbExecutor};

/// creates and returns the app after mounting all routes/resources
pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {
    App::with_state(AppState { db })
        .middleware(Logger::default())
        // everything under '/api/' route

//        .resource("/likes", |r| {
//            r.method(Method::POST).with(add_like);
//        })
        .resource("/likes/{id}", |r| {
            r.method(Method::GET).with(get_likes);
            r.method(Method::POST).with(add_like);
        })
}
