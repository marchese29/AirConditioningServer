use rocket::serde::Serialize;

use super::{JoinType, WebhookDescription};

#[derive(Serialize)]
pub enum Status {
    On,
    Off,
}

#[derive(Serialize)]
pub struct ConditionDescription {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub status: Status,
}

#[derive(Serialize)]
pub struct ShallowAction {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub webhooks: WebhookDescription,
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
