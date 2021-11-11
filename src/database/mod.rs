use rocket_sync_db_pools::{database, diesel};

pub mod access;
pub mod models;
pub mod schema;

#[database("sqlite_conditions")]
pub struct ConditionDbConn(diesel::SqliteConnection);
