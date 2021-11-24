use diesel::result::Error as DieselError;
use rocket::{http::Status, response::Responder, serde::json::Json};

pub type ACResult<T> = std::result::Result<T, ACError>;
pub type ACApiResult<T> = std::result::Result<Json<T>, ACError>;

#[derive(Debug)]
pub enum ACError {
    InternalServerError,
    DatabaseAccessError(String),
    DataEntryMissingError,
}

impl<'r> Responder<'r, 'static> for ACError {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            ACError::InternalServerError => Err(Status::InternalServerError),
            ACError::DatabaseAccessError(_) => Err(Status::InternalServerError),
            ACError::DataEntryMissingError => Err(Status::NotFound),
        }
    }
}

impl std::fmt::Display for ACError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ACError::InternalServerError => write!(f, "Internal Server Error"),
            ACError::DatabaseAccessError(err) => write!(f, "Error accessing database: {}", err),
            ACError::DataEntryMissingError => write!(f, "Data entry was missing"),
        }
    }
}
impl std::error::Error for ACError {}

impl From<DieselError> for ACError {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::DatabaseError(_, info) => {
                ACError::DatabaseAccessError(format!("{}", info.message()))
            }
            DieselError::NotFound => ACError::DataEntryMissingError,
            _ => ACError::InternalServerError,
        }
    }
}
