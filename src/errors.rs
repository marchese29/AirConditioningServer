use std::{error, fmt};

use rocket::response::Responder;

pub type ACResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// #[derive(Debug, Clone, Responder)]
// #[response(status = 500, content_type = "plain")]
// pub struct DatabaseAccessError {
//     message: String,
// }

// impl DatabaseAccessError {
//     pub fn with_message(message: &str) -> Self {
//         Self {
//             message: message.to_string(),
//         }
//     }
// }

// impl fmt::Display for DatabaseAccessError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "Encountered p roblem accessing the database: {}",
//             &self.message
//         )
//     }
// }

// impl error::Error for DatabaseAccessError {}

#[derive(Debug, Clone, Responder)]
#[response(status = 404, content_type = "plain")]
pub struct DataEntryMissingError {
    message: String,
}

impl DataEntryMissingError {
    pub fn with_message(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for DataEntryMissingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing data entry: {}", &self.message)
    }
}

impl error::Error for DataEntryMissingError {}
