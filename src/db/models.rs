use diesel::prelude::*;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

use crate::db::schema::posts;
use crate::errors::Error;
use crate::errors::Result;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Default)]
#[diesel(table_name = posts)]
pub(crate) struct Post {
    pub id: String,
    pub likes: i32,
}

impl Post {
    pub(crate) fn find_by_id(conn: &mut SqliteConnection, id: &str) -> Result<Post> {
        posts::table
            .filter(posts::id.eq(id))
            .get_result::<Post>(conn)
            .map_err(|e| e.into())
    }

    pub(crate) fn insert_or_increment_like(conn: &mut SqliteConnection, id: &str) -> Result<Post> {
        let updated = diesel::update(posts::table.find(id))
            .set(posts::likes.eq(posts::likes + 1))
            .execute(conn)
            .unwrap_or(0);

        if updated == 1 {
            // If the update was successful (1 row updated), fetch and return the updated post
            Post::find_by_id(conn, id)
        } else {
            // If no rows were updated, insert a new post with 'likes' count set to 1
            match diesel::insert_into(posts::table)
                .values((posts::likes.eq(1), posts::id.eq(id)))
                .execute(conn)
            {
                Ok(_) => Post::find_by_id(conn, id), // Insertion successful, fetch and return the new post
                Err(_) => Err(Error::internal("Couldn't insert into table")), // Insertion failed, return an error
            }
        }
    }
}
