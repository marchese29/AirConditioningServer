use rocket::serde::{Deserialize, Serialize};

pub mod request;
pub mod response;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum JoinType {
    Any,
    All,
}
