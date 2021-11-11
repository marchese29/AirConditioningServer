use diesel::prelude::*;

use super::models::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn get_actions(conn: &rocket_sync_db_pools::diesel::SqliteConnection) -> Result<Vec<Trigger>> {
    use super::schema::triggers::dsl::*;
    Ok(triggers.filter(action_name.is_not_null()).order_by(id.asc()).load::<Trigger>(conn)?)
}

pub fn get_conditions(conn: &rocket_sync_db_pools::diesel::SqliteConnection) -> Result<Vec<Condition>> {
    use super::schema::conditions::dsl::*;
    Ok(conditions.filter(name.is_not_null()).order_by(id.asc()).load::<Condition>(conn)?)
}