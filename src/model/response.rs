use rocket::serde::Serialize;

use crate::database::models::{Condition, Trigger};

use super::{JoinType, WebhookDescription};

#[derive(Serialize)]
pub enum Status {
    On,
    Off,
}

#[derive(Serialize)]
pub struct ConditionDescription {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl ConditionDescription {
    pub fn from_condition(condition: &Condition) -> Self {
        Self {
            id: condition.id,
            name: condition.name.as_ref().unwrap().to_string(),
            description: condition.description.as_ref().unwrap().to_string(),
            status: if condition.is_on { Status::On } else { Status::Off }
        }
    }
}

#[derive(Serialize)]
pub struct ShallowAction {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl ShallowAction {
    pub fn from_trigger(trigger: &Trigger) -> Self {
        Self {
            id: trigger.id,
            name: trigger.action_name.as_ref().unwrap().to_string(),
            description: trigger.action_description.as_ref().unwrap().to_string(),
        }
    }
}

#[derive(Serialize)]
pub enum Component {
    Join(JoinDescription),
    Condition(ConditionDescription),
}

#[derive(Serialize)]
pub struct JoinDescription {
    pub components: Vec<Component>,
    pub join_type: JoinType,
}

#[derive(Serialize)]
pub struct ActionDescription {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub webhooks: WebhookDescription,
    pub join_description: JoinDescription,
}
