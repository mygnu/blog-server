use actix::{Handler, Message};
use actix_web::{web::Data, web::Path, Error, HttpResponse, ResponseError};
use diesel::prelude::*;
use futures::Future;
use serde::Deserialize;

use crate::db::models::{AppData, DbExecutor, Post};

use super::errors::ServiceError;

#[derive(Debug, Default, Deserialize)]
pub struct AddLike(String);

impl Message for AddLike {
    type Result = Result<Post, ServiceError>;
}

impl Handler<AddLike> for DbExecutor {
    type Result = Result<Post, ServiceError>;

    fn handle(&mut self, msg: AddLike, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::posts::dsl::{id, likes, posts};
        let conn: &SqliteConnection = &self.0.get().unwrap();

        let updated = diesel::update(posts.find(&msg.0))
            .set(likes.eq(likes + 1))
            .execute(conn)
            .unwrap_or(0);

        if updated == 1 {
            get_post_by_id(conn, &msg.0)
        } else {
            let inserted = diesel::insert_into(posts)
                .values((likes.eq(1), id.eq(&msg.0)))
                .execute(conn);
            if inserted.is_err() {
                Err(ServiceError::BadRequest(
                    "Couldn't insert into table".into(),
                ))
            } else {
                get_post_by_id(conn, &msg.0)
            }
        }
    }
}

fn get_post_by_id(conn: &SqliteConnection, post_id: &str) -> Result<Post, ServiceError> {
    use crate::db::schema::posts::dsl::posts;
    let result = posts.find(post_id).get_result::<Post>(conn);
    match result {
        Ok(like) => Ok(like),
        Err(_) => Err(ServiceError::BadRequest(format!(
            "Can't find id: {}",
            post_id
        ))),
    }
}

pub fn add_like(
    id: Path<String>,
    state: Data<AppData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    state
        .db
        .send(AddLike(id.into_inner()))
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(like) => Ok(HttpResponse::Ok().json(like)),
            Err(service_error) => Ok(service_error.error_response()),
        })
}

pub struct GetPost(String);

impl Message for GetPost {
    type Result = Result<Post, ServiceError>;
}

impl Handler<GetPost> for DbExecutor {
    type Result = Result<Post, ServiceError>;
    fn handle(&mut self, msg: GetPost, _: &mut Self::Context) -> Self::Result {
        let conn: &SqliteConnection = &self.0.get().unwrap();
        get_post_by_id(conn, &msg.0)
    }
}

pub fn get_post(
    id: Path<String>,
    state: Data<AppData>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    state
        .db
        .send(GetPost(id.into_inner()))
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(like) => Ok(HttpResponse::Ok().json(like)),
            Err(service_error) => Ok(service_error.error_response()),
        })
}
