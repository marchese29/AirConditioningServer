use rocket_sync_db_pools::diesel::SqliteConnection;

use crate::{errors::ACResult, model::response::TriggerDescription};

pub fn traverse_trigger(
    trigger_id: i32,
    conn: &SqliteConnection,
) -> ACResult<Option<TriggerDescription>> {
    todo!()
}
