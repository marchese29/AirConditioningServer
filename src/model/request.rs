use rocket::serde::Deserialize;

use super::{JoinType, WebhookDescription};

#[derive(Deserialize)]
pub struct CreateConditionRequest {
    pub name: String,
    pub description: String,
    pub is_on: bool,
}

#[derive(Deserialize)]
pub struct CreateActionRequest {
    pub name: String,
    pub description: String,
    pub webhooks: WebhookDescription,
    pub join: JoinRequest,
}

#[derive(Deserialize)]
pub struct JoinRequest {
    pub components: Vec<ActionComponent>,
    pub join_type: JoinType,
}

#[derive(Deserialize)]
pub enum ActionComponent {
    Join(JoinRequest),
    ExistingCondition(u32),
    NewCondition(CreateConditionRequest),
}
