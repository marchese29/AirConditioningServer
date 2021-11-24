use crate::{
    database::{access::*, models::*},
    errors::ACResult,
    logic::describe::traverse_trigger,
    model::{
        request::{CreateTriggerRequest, TriggerComponent},
        response::{Component, ConditionDescription, TriggerDescription},
        JoinType,
    },
};

use rocket_sync_db_pools::diesel::SqliteConnection;

pub fn assemble_trigger(
    request: &CreateTriggerRequest,
    conn: &SqliteConnection,
) -> ACResult<TriggerDescription> {
    let new_trigger = NewTrigger {
        name: request.name.clone(),
        description: request.description.clone(),
        needs_all: matches!(request.join_type, JoinType::All),
    };
    let trigger = create_trigger(conn, new_trigger)?;

    let mut components = Vec::new();
    for component in request.components.iter() {
        match component {
            TriggerComponent::NewCondition(condition_request) => {
                let new_condition = NewCondition {
                    name: condition_request.name.clone(),
                    description: condition_request.description.clone(),
                    is_on: condition_request.is_on,
                };
                let condition = create_condition(conn, new_condition)?;

                let trigger_condition = NewTriggerCondition {
                    triggered_id: trigger.id,
                    condition_id: condition.id,
                };
                create_trigger_condition(conn, &trigger_condition)?;
                components.push(Component::Condition(ConditionDescription::from_condition(
                    &condition,
                )));
            }
            TriggerComponent::NewTrigger(trigger_request) => {
                let new_trigger = assemble_trigger(trigger_request, conn)?;

                let trigger_trigger = NewTriggerTrigger {
                    triggered_id: trigger.id,
                    triggering_id: new_trigger.id,
                };
                create_trigger_trigger(conn, &trigger_trigger)?;
                components.push(Component::Trigger(new_trigger));
            }
            TriggerComponent::ExistingCondition(condition_id) => {
                let condition = get_condition_for_id(conn, *condition_id)?;
                let trigger_condition = NewTriggerCondition {
                    triggered_id: trigger.id,
                    condition_id: condition.id,
                };
                create_trigger_condition(conn, &trigger_condition)?;
                components.push(Component::Condition(ConditionDescription::from_condition(
                    &condition,
                )));
            }
            TriggerComponent::ExistingTrigger(trigger_id) => {
                let new_trigger = traverse_trigger(*trigger_id, conn)?;
                let trigger_trigger = NewTriggerTrigger {
                    triggered_id: trigger.id,
                    triggering_id: new_trigger.id,
                };
                create_trigger_trigger(conn, &trigger_trigger)?;
                components.push(Component::Trigger(new_trigger));
            }
        }
    }

    let mut statuses = Vec::new();
    for component in components.iter() {
        match component {
            Component::Condition(condition) => {
                statuses.push(condition.is_on);
            }
            Component::Trigger(trigger) => {
                statuses.push(trigger.is_on);
            }
        }
    }

    let is_on: bool;
    if trigger.needs_all {
        is_on = statuses.iter().all(|&s| s);
    } else {
        is_on = statuses.iter().any(|&s| s);
    }

    Ok(TriggerDescription {
        id: trigger.id,
        name: trigger.name.clone(),
        description: trigger.description.clone(),
        components,
        join_type: request.join_type,
        is_on,
    })
}
