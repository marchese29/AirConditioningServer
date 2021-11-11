use std::{fmt, error};

use rocket::response::Responder;

#[derive(Debug, Clone, Responder)]
#[response(status = 500, content_type = "plain")]
pub struct DatabaseAccessError {
    message: String,
}

impl DatabaseAccessError {
    pub fn with_message(message: &str) -> Self {
        Self { message: message.to_string() }
    }
}

impl fmt::Display for DatabaseAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Encountered problem accessing the database: {}", &self.message)
    }
}

impl error::Error for DatabaseAccessError { }