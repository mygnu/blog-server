use actix_web::{web::Data, web::Path, HttpResponse};

use super::tracing_block;
use crate::{db::models::Post, errors::Result, AppState};

#[actix_web::get("/api/posts/{id}")]
#[tracing::instrument(err, skip_all, level = "debug")]
pub(crate) async fn get_likes(id: Path<String>, state: Data<AppState>) -> Result<HttpResponse> {
    let pool = state.pool.clone();

    let post = tracing_block(move || -> Result<Post> {
        let id = id.into_inner();
        let conn = &mut pool.get()?;
        let post = Post::find_by_id(conn, &id)?;
        Ok(post)
    })
    .await??;

    Ok(HttpResponse::Ok().json(post))
}

#[actix_web::post("/api/posts/{id}")]
#[tracing::instrument(err, skip_all, level = "debug")]
pub(crate) async fn like_post(id: Path<String>, state: Data<AppState>) -> Result<HttpResponse> {
    let pool = state.pool.clone();

    let post = tracing_block(move || -> Result<Post> {
        let conn = &mut pool.get()?;
        let id = id.into_inner();
        let post = Post::insert_or_increment_like(conn, &id)?;
        Ok(post)
    })
    .await??;

    Ok(HttpResponse::Ok().json(post))
}
