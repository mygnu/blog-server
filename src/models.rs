use actix::{Actor, Addr, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::schema::likes;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

// Actors communicate exclusively by exchanging messages.
// The sending actor can optionally wait for the response.
// Actors are not referenced directly, but by means of addresses.
// Any rust type can be an actor, it only needs to implement the Actor trait.
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Default)]
#[table_name = "likes"]
pub struct Like {
    pub id: i32,
    pub value: i32,
}
