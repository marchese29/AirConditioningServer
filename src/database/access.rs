use diesel::{insert_into, prelude::*, select, sql_types};
use rocket_sync_db_pools::diesel::SqliteConnection;

use crate::errors::ACResult;

use super::models::*;

no_arg_sql_function!(last_insert_id, sql_types::Integer);

pub fn get_actions(conn: &SqliteConnection) -> ACResult<Vec<Trigger>> {
    use super::schema::triggers::dsl::*;
    Ok(triggers.order_by(id.asc()).load::<Trigger>(conn)?)
}

pub fn get_conditions(conn: &SqliteConnection) -> ACResult<Vec<Condition>> {
    use super::schema::conditions::dsl::*;
    Ok(conditions.order_by(id.asc()).load::<Condition>(conn)?)
}

pub fn get_condition_for_id(
    conn: &SqliteConnection,
    condition_id: i32,
) -> ACResult<Option<Condition>> {
    use super::schema::conditions::dsl::*;
    Ok(conditions
        .find(condition_id)
        .first::<Condition>(conn)
        .optional()?)
}

pub fn get_trigger_for_id(conn: &SqliteConnection, trigger_id: i32) -> ACResult<Option<Trigger>> {
    use super::schema::triggers::dsl::*;
    Ok(triggers
        .find(trigger_id)
        .first::<Trigger>(conn)
        .optional()?)
}

pub fn get_conditions_for_trigger(
    conn: &SqliteConnection,
    trigger: &Trigger,
) -> ACResult<Vec<Condition>> {
    use super::schema::conditions::dsl::*;
    let tconds: Vec<i32> = TriggerCondition::belonging_to(trigger)
        .load::<TriggerCondition>(conn)?
        .iter()
        .map(|tc| tc.condition_id)
        .collect();
    Ok(conditions
        .filter(id.eq_any(tconds))
        .load::<Condition>(conn)?)
}

pub fn get_triggers_for_trigger(
    conn: &SqliteConnection,
    trigger: &Trigger,
) -> ACResult<Vec<Trigger>> {
    use super::schema::triggers::dsl::*;
    let ttrigs: Vec<i32> = TriggerTrigger::belonging_to(trigger)
        .load::<TriggerTrigger>(conn)?
        .iter()
        .map(|tt| tt.triggering_id)
        .collect();
    Ok(triggers.filter(id.eq_any(ttrigs)).load::<Trigger>(conn)?)
}

pub fn get_webhook_for_trigger(
    conn: &SqliteConnection,
    trigger: &Trigger,
) -> ACResult<Option<Webhook>> {
    Ok(Webhook::belonging_to(trigger)
        .first::<Webhook>(conn)
        .optional()?)
}

pub fn create_condition(
    conn: &SqliteConnection,
    new_condition: NewCondition,
) -> ACResult<Condition> {
    use super::schema::conditions::dsl::*;
    insert_into(conditions)
        .values(&new_condition)
        .execute(conn)?;

    let last_id: i32 = select(last_insert_id).first(conn)?;
    Ok(conditions.find(last_id).first::<Condition>(conn)?)
}

pub fn create_trigger(conn: &SqliteConnection, new_trigger: NewTrigger) -> ACResult<Trigger> {
    use super::schema::triggers::dsl::*;
    insert_into(triggers).values(&new_trigger).execute(conn)?;

    let last_id: i32 = select(last_insert_id).first(conn)?;
    Ok(triggers.find(last_id).first::<Trigger>(conn)?)
}

pub fn create_webhook(conn: &SqliteConnection, new_webhook: &NewWebhook) -> ACResult<()> {
    use super::schema::webhooks::dsl::*;
    insert_into(webhooks).values(new_webhook).execute(conn)?;
    Ok(())
}

pub fn create_trigger_condition(
    conn: &SqliteConnection,
    trigger_condition: &NewTriggerCondition,
) -> ACResult<()> {
    use super::schema::trigger_conditions::dsl::*;
    insert_into(trigger_conditions)
        .values(trigger_condition)
        .execute(conn)?;
    Ok(())
}

pub fn create_trigger_trigger(
    conn: &SqliteConnection,
    trigger_trigger: &NewTriggerTrigger,
) -> ACResult<()> {
    use super::schema::trigger_triggers::dsl::*;
    insert_into(trigger_triggers)
        .values(trigger_trigger)
        .execute(conn)?;
    Ok(())
}
