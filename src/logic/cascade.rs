use rocket_sync_db_pools::diesel::SqliteConnection;

use crate::{
    database::{
        access::{
            get_conditions_for_trigger, get_triggered_triggers, get_triggering_triggers,
            get_triggers_for_condition, set_condition_off, set_condition_on,
        },
        models::{Condition, Trigger},
    },
    errors::ACResult,
    logic::describe::traverse_trigger_object,
    model::response::TriggerDescription,
};

pub fn turn_condition_on(condition: &Condition, conn: &SqliteConnection) -> ACResult<()> {
    if condition.is_on {
        // Nothing to do
        return Ok(());
    }

    for trigger in get_triggers_for_condition(conn, condition)?.iter() {
        set_condition_for_trigger(condition, trigger, conn)?;
    }

    set_condition_on(conn, condition)?;
    Ok(())
}

fn set_condition_for_trigger(
    condition_to_set: &Condition,
    trigger: &Trigger,
    conn: &SqliteConnection,
) -> ACResult<()> {
    let (conditions, triggers, is_already_on) = describe_current_state(trigger, conn)?;

    if !is_already_on {
        if !trigger.needs_all {
            // It only takes one condition to turn this on
            engage_trigger(trigger, conn)?;
        } else {
            if triggers.iter().all(|t| t.is_on) {
                let mut condition_states = Vec::new();
                for condition in conditions.iter() {
                    if condition.eq(condition_to_set) {
                        condition_states.push(true);
                    } else {
                        condition_states.push(condition.is_on);
                    }
                }

                if condition_states.iter().all(|cs| *cs) {
                    engage_trigger(trigger, conn)?;
                }
            }
        }
    }
    Ok(())
}

fn set_trigger_for_trigger(
    trigger_to_set: &Trigger,
    trigger: &Trigger,
    conn: &SqliteConnection,
) -> ACResult<()> {
    let (conditions, triggers, is_already_on) = describe_current_state(trigger, conn)?;

    if !is_already_on {
        if !trigger.needs_all {
            // It only takes one trigger to turn this on
            engage_trigger(trigger, conn)?;
        } else {
            // Is this the last item we need?
            if conditions.iter().all(|c| c.is_on) {
                let mut trigger_states = Vec::new();
                for t in triggers.iter() {
                    if t.id == trigger_to_set.id {
                        trigger_states.push(true);
                    } else {
                        trigger_states.push(t.is_on);
                    }
                }

                if trigger_states.iter().all(|ts| *ts) {
                    engage_trigger(trigger, conn)?;
                }
            }
        }
    }
    Ok(())
}

pub fn turn_condition_off(condition: &Condition, conn: &SqliteConnection) -> ACResult<()> {
    if !condition.is_on {
        // Nothing to do
        return Ok(());
    }

    for trigger in get_triggers_for_condition(conn, condition)?.iter() {
        unset_condition_for_trigger(condition, trigger, conn)?;
    }

    set_condition_off(conn, condition)?;
    Ok(())
}

fn unset_condition_for_trigger(
    condition_to_unset: &Condition,
    trigger: &Trigger,
    conn: &SqliteConnection,
) -> ACResult<()> {
    let (conditions, triggers, is_currently_on) = describe_current_state(trigger, conn)?;

    if is_currently_on {
        if trigger.needs_all {
            // It only takes one condition to turn this off
            disengage_trigger(trigger, conn)?;
        } else {
            // Is this the last thing to shut off?
            if triggers.iter().all(|t| !t.is_on) {
                let mut condition_states = Vec::new();
                for condition in conditions.iter() {
                    if condition.eq(condition_to_unset) {
                        condition_states.push(false);
                    } else {
                        condition_states.push(condition.is_on);
                    }
                }

                if condition_states.iter().all(|cs| !*cs) {
                    disengage_trigger(trigger, conn)?;
                }
            }
        }
    }
    Ok(())
}

fn unset_trigger_for_trigger(
    trigger_to_unset: &Trigger,
    trigger: &Trigger,
    conn: &SqliteConnection,
) -> ACResult<()> {
    let (conditions, triggers, is_currently_on) = describe_current_state(trigger, conn)?;

    if is_currently_on {
        if trigger.needs_all {
            // It only takes one trigger to turn this on
            disengage_trigger(trigger, conn)?;
        } else {
            // Is this the last item we need?
            if conditions.iter().all(|c| !c.is_on) {
                let mut trigger_states = Vec::new();
                for t in triggers.iter() {
                    if t.id == trigger_to_unset.id {
                        trigger_states.push(false);
                    } else {
                        trigger_states.push(t.is_on);
                    }
                }

                if trigger_states.iter().all(|ts| !*ts) {
                    disengage_trigger(trigger, conn)?;
                }
            }
        }
    }
    Ok(())
}

fn describe_current_state(
    trigger: &Trigger,
    conn: &SqliteConnection,
) -> ACResult<(Vec<Condition>, Vec<TriggerDescription>, bool)> {
    let conditions = get_conditions_for_trigger(conn, trigger)?;
    let mut triggers = Vec::new();
    for triggering in get_triggering_triggers(conn, trigger)?.iter() {
        triggers.push(traverse_trigger_object(triggering, conn)?);
    }

    let is_already_on = if !trigger.needs_all {
        !conditions.iter().any(|c| c.is_on) || !triggers.iter().any(|t| t.is_on)
    } else {
        false
    };

    Ok((conditions, triggers, is_already_on))
}

fn engage_trigger(trigger: &Trigger, conn: &SqliteConnection) -> ACResult<()> {
    for triggered in get_triggered_triggers(conn, trigger)?.iter() {
        set_trigger_for_trigger(trigger, triggered, conn)?;
    }
    todo!("Carry out actions")
}

fn disengage_trigger(trigger: &Trigger, conn: &SqliteConnection) -> ACResult<()> {
    for triggered in get_triggered_triggers(conn, trigger)?.iter() {
        unset_trigger_for_trigger(trigger, triggered, conn)?;
    }
    todo!("Carry out actions")
}
