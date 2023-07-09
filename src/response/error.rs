use serde::{Serialize, Deserialize};

use super::Type;


///Error struct for returning to the http response.
/// Error type shows the error type and reason the more detail reason
#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    //TODO types of error
    pub error_type: Type,
    pub reason: String,
}

impl Error {
    pub fn new(error_type: Type, reason: String) -> Self {
        Error { error_type, reason }
    }
}
