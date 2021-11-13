use rocket::serde::{Deserialize, Serialize};

use crate::database::models::Webhook;

pub mod request;
pub mod response;

#[derive(Serialize, Deserialize, Clone)]
pub struct WebhookDescription {
    pub engaged_webhook: String,
    pub disengaged_webhook: Option<String>,
}

impl WebhookDescription {
    pub fn from_webhook(webhook: &Webhook) -> Self {
        Self {
            engaged_webhook: webhook.engage_url.clone(),
            disengaged_webhook: webhook.disengage_url.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum JoinType {
    Any,
    All,
}
