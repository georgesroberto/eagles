// src/db.rs

use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_connection(pool: &DbPool) -> SqliteConnection {
    pool.get().expect("Failed to get a connection from the pool")
}
