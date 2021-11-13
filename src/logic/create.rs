use crate::{
    database::{access::*, models::*},
    errors::{ACResult, DataEntryMissingError},
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
                if let Some(condition) = get_condition_for_id(conn, *condition_id)? {
                    let trigger_condition = NewTriggerCondition {
                        triggered_id: trigger.id,
                        condition_id: condition.id,
                    };
                    create_trigger_condition(conn, &trigger_condition)?;
                    components.push(Component::Condition(ConditionDescription::from_condition(
                        &condition,
                    )));
                } else {
                    return Err(Box::new(DataEntryMissingError::with_message(format!(
                        "Condition with id {}",
                        *condition_id
                    ))));
                }
            }
            TriggerComponent::ExistingTrigger(trigger_id) => {
                if let Some(new_trigger) = traverse_trigger(*trigger_id, conn)? {
                    let trigger_trigger = NewTriggerTrigger {
                        triggered_id: trigger.id,
                        triggering_id: new_trigger.id,
                    };
                    create_trigger_trigger(conn, &trigger_trigger)?;
                    components.push(Component::Trigger(new_trigger));
                } else {
                    return Err(Box::new(DataEntryMissingError::with_message(format!(
                        "Trigger with id {}",
                        *trigger_id
                    ))));
                }
            }
        }
    }

    if let Some(webhook) = &request.webhooks {
        let new_webhook = NewWebhook {
            engage_url: webhook.engaged_webhook.clone(),
            disengage_url: webhook.disengaged_webhook.clone(),
            trigger_id: trigger.id,
        };
        create_webhook(conn, &new_webhook)?;
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
        webhooks: request.webhooks.clone(),
        components,
        join_type: request.join_type,
        is_on,
    })
}
