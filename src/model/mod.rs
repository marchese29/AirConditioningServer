use rocket::serde::{Deserialize, Serialize};

pub mod request;
pub mod response;

#[derive(Serialize, Deserialize)]
pub struct WebhookDescription {
    pub engaged_webhook: String,
    pub disengaged_webhook: String,
}

#[derive(Serialize, Deserialize)]
pub enum JoinType {
    Any,
    All,
}
