use rocket::serde::Serialize;

use crate::database::models::{Condition, Trigger};

use super::JoinType;

#[derive(Serialize)]
pub struct ConditionDescription {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub is_on: bool,
}

impl ConditionDescription {
    pub fn from_condition(condition: &Condition) -> Self {
        Self {
            id: condition.id,
            name: condition.name.clone(),
            description: condition.description.clone(),
            is_on: condition.is_on,
        }
    }
}

#[derive(Serialize)]
pub struct ShallowTrigger {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl ShallowTrigger {
    pub fn from_trigger(trigger: &Trigger) -> Self {
        Self {
            id: trigger.id,
            name: trigger.name.clone(),
            description: trigger.description.clone(),
        }
    }
}

#[derive(Serialize)]
pub enum Component {
    Trigger(TriggerDescription),
    Condition(ConditionDescription),
}

#[derive(Serialize)]
pub struct TriggerDescription {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub components: Vec<Component>,
    pub join_type: JoinType,
    pub is_on: bool,
}
