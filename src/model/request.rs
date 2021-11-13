use rocket::serde::Deserialize;

use super::{JoinType, WebhookDescription};

#[derive(Deserialize)]
pub struct CreateConditionRequest {
    pub name: String,
    pub description: String,
    pub is_on: bool,
}

#[derive(Deserialize)]
pub struct CreateTriggerRequest {
    pub name: String,
    pub description: String,
    pub webhooks: Option<WebhookDescription>,
    pub components: Vec<TriggerComponent>,
    pub join_type: JoinType,
}

#[derive(Deserialize)]
pub enum TriggerComponent {
    NewTrigger(CreateTriggerRequest),
    ExistingTrigger(i32),
    NewCondition(CreateConditionRequest),
    ExistingCondition(i32),
}
