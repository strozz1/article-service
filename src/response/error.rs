use serde::{Deserialize, Serialize};

use super::RequestStatus;

///Error struct for returning to the http response.
/// Error type shows the error type and reason the more detail reason
#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    //TODO types of error
    pub code: u8,
    pub description: String,
    pub reason: String,
}

impl Error {
    pub fn new(code_type: ErrorType, reason: String) -> Self {
        Error {
            code: code_type.value(),
            description: code_type.description(),
            reason,
        }
    }
}

pub enum ErrorType{
    InvalidId,
    MalformedJSON,
    NotFound,
    Internal,
    DuplicateKey ,
    Timeout
}
impl ErrorType{
    pub fn value(&self)-> u8{
        match *self {
            ErrorType::InvalidId => 0,
            ErrorType::NotFound => 1,
            ErrorType::MalformedJSON => 2,
            ErrorType::Internal => 3,
            ErrorType::DuplicateKey => 4,
            ErrorType::Timeout => 5
        }
    }

    pub fn description(&self)-> String{
        match *self {
            ErrorType::InvalidId => "Invalid id".to_string(),
            ErrorType::NotFound => "Not found".to_string(),
            ErrorType::MalformedJSON =>"Malformed JSON".to_string(),
            ErrorType::Internal => "Internal server error".to_string(),
            ErrorType::DuplicateKey => "Duplicated key from request".to_string(),
            ErrorType::Timeout => "Request timeout".to_string()
        }
    }
}
