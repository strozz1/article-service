use mongodb::bson::DateTime;
use serde::{Serialize, Deserialize};

pub mod error;
pub mod accept;

///This trait is the basic response for a http request. This struct will transform into a JSON format with serde crate and will be sent as http response (JSON).
///
/// The struct consists in diferent atributes
/// A code for the response tipe: Error, Ok, BadRequest...
/// The description of the response.
///  the content, either a json error or the json response object
/// Timestamp: date and time from the request
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    code: u8,
    description: String,
    timestamp: DateTime,
    content_size: usize,
    content: Vec<T>,
}

impl<T> Response<T> {
    ///Returns a new instance of Response struct with a type and a content
    ///
    /// Content should implement Serialize and Deserialize
    pub fn new(response_type: Type, content: T) -> Self {
        let code = response_type.value();
        let description = response_type.description().to_string();

        let mut vector = Vec::new();
        vector.push(content);

        let timestamp = DateTime::now();
        Response {
            code,
            description,
            timestamp,
            content_size: vector.len(),
            content: vector,
        }
    }

    pub fn new_from_multiple(response_type: Type, vector: Vec<T>) -> Self {
        let code = response_type.value();
        let description = response_type.description().to_string();
        let timestamp = DateTime::now();
        Response {
            code,
            description,
            timestamp,
            content_size: vector.len(),
            content: vector,
        }
    }
}

///Type enum represents the response type.
/// Has 2 methods
///
/// One for getting the int value of the code and another one for the description
#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    //TODO change this to http type response
    Ok,
    BadRequest,
    MalformedJSON,
    Database,
    Internal,
    DuplicateKey,
    Write,
}
impl Type {
    /// Returns the value from the code
    pub fn value(&self) -> u8 {
        match *self {
            Type::Ok => 0,
            Type::BadRequest => 1,
            Type::MalformedJSON => 2,
            Type::Database => 3,
            Type::Internal => 4,
            Type::DuplicateKey => 5,
            Type::Write => 6,
        }
    }
    /// returns the description for each response
    pub fn description(&self) -> &str {
        match *self {
            Type::Ok => "Response is correct",
            Type::BadRequest => "The request is not correctly formed.",
            Type::MalformedJSON => "The JSON is malformed",
            Type::Database => "There is a database error",
            Type::Internal => "Internal server error occurred",
            Type::DuplicateKey => "Duplicate Key conflict",
            Type::Write => "There is a write error",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestId {
    pub id: String,
}
