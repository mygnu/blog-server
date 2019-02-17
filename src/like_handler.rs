use actix::{Handler, Message};
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Path, ResponseError, State};
use diesel::{self, prelude::*};
use futures::future::Future;

use crate::errors::ServiceError;
use crate::models::{AppState, DbExecutor, Like};

#[derive(Debug, Default, Deserialize)]
pub struct AddLike(i32);

impl Message for AddLike {
    type Result = Result<Like, ServiceError>;
}

impl Handler<AddLike> for DbExecutor {
    type Result = Result<Like, ServiceError>;
    fn handle(&mut self, msg: AddLike, _: &mut Self::Context) -> Self::Result {
        use crate::schema::likes::dsl::{likes, id, value};
        let conn: &PgConnection = &self.0.get().unwrap();

        let updated: Result<Like, _> = diesel::update(likes)
            .filter(id.eq(msg.0))
            .set(value.eq(value + 1))
            .get_result(conn);

        match updated {
            Ok(like) => Ok(like),
            Err(_) => {
                diesel::insert_into(likes)
                    .values(value.eq(1))
                    .get_result(conn)
                    .map_err(|diesel_error| diesel_error.into())
            }
        }
    }
}

pub fn add_like((id, state): (Path<i32>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db
        .send(AddLike(id.into_inner()))
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(like) => Ok(HttpResponse::Ok().json(like)),
            Err(service_error) => Ok(service_error.error_response()),
        }).responder()
}


pub struct GetLike(i32);

impl Message for GetLike {
    type Result = Result<Like, ServiceError>;
}

impl Handler<GetLike> for DbExecutor {
    type Result = Result<Like, ServiceError>;
    fn handle(&mut self, msg: GetLike, _: &mut Self::Context) -> Self::Result {
        use crate::schema::likes::dsl::likes;
        let conn: &PgConnection = &self.0.get().unwrap();

        return likes
            .find(msg.0)
            .get_result(conn)
            .map_err(|diesel_error| diesel_error.into());
    }
}

pub fn get_likes((id, state): (Path<i32>, State<AppState>)) -> FutureResponse<HttpResponse> {
    let get_like = GetLike(id.into_inner());
    state.db
        .send(get_like)
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(like) => Ok(HttpResponse::Ok().json(like)),
            Err(service_error) => Ok(service_error.error_response()),
        }).responder()
}
