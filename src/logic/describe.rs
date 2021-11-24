use rocket_sync_db_pools::diesel::SqliteConnection;

use crate::{
    database::{
        access::{
            get_conditions_for_trigger, get_trigger_for_id, get_triggering_triggers,
            get_webhook_for_trigger,
        },
        models::Trigger,
    },
    errors::ACResult,
    model::{
        response::{Component, ConditionDescription, TriggerDescription},
        JoinType, WebhookDescription,
    },
};

pub fn traverse_trigger(
    trigger_id: i32,
    conn: &SqliteConnection,
) -> ACResult<TriggerDescription> {
    Ok(traverse_trigger_object(&get_trigger_for_id(conn, trigger_id)?, conn)?)
}

pub fn traverse_trigger_object(
    trigger: &Trigger,
    conn: &SqliteConnection,
) -> ACResult<TriggerDescription> {
    let mut components = Vec::new();
    for condition in get_conditions_for_trigger(conn, &trigger)? {
        let description = ConditionDescription {
            id: condition.id,
            name: condition.name.clone(),
            description: condition.description.clone(),
            is_on: condition.is_on,
        };
        components.push(Component::Condition(description));
    }
    for triggerer in get_triggering_triggers(conn, &trigger)? {
        let description = traverse_trigger_object(&triggerer, conn)?;
        components.push(Component::Trigger(description));
    }

    let is_on: bool;
    if trigger.needs_all {
        is_on = components
            .iter()
            .map(|c| match c {
                Component::Condition(cond) => cond.is_on,
                Component::Trigger(trig) => trig.is_on,
            })
            .all(|x| x);
    } else {
        is_on = components
            .iter()
            .map(|c| match c {
                Component::Condition(cond) => cond.is_on,
                Component::Trigger(trig) => trig.is_on,
            })
            .any(|x| x);
    }

    Ok(TriggerDescription {
        id: trigger.id,
        name: trigger.name.clone(),
        description: trigger.description.clone(),
        webhooks: get_webhook_for_trigger(conn, &trigger)?
            .as_ref()
            .map(WebhookDescription::from_webhook),
        components,
        join_type: if trigger.needs_all {
            JoinType::All
        } else {
            JoinType::Any
        },
        is_on,
    })
}
