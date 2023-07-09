pub mod response {
    use mongodb::bson::DateTime;
    use serde::{Deserialize, Serialize};

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
        content: T,
    }

    impl<T> Response<T> {
        ///Returns a new instance of Response struct with a type and a content
        ///
        /// Content should implement Serialize and Deserialize
        pub fn new(response_type: Type, content: T) -> Self {
            let code = response_type.value();
            let description = response_type.description().to_string();

            let timestamp = DateTime::now();
            Response {
                code,
                description,
                timestamp,
                content,
            }
        }
    }

///Type enum represents the response type.
/// Has 2 methods
/// 
/// One for getting the int value of the code and another one for the description
    #[derive(Serialize, Deserialize, Debug)]
    pub enum Type {
        Ok,
        BadRequest,
        MalformedJSON,
        Database,
        Internal,
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
            }
        }
        /// returns the description for each response
        pub fn description(&self) -> &str {
            match *self {
                Type::Ok => "Response is correct",
                Type::BadRequest => "The request is not correctly formed.",
                Type::MalformedJSON => "The JSON is malformed",
                Type::Database => "Database error",
                Type::Internal => "Internal server error",
            }
        }
    }


    ///Error struct for returning to the http response.
    /// Error type shows the error type and reason the more detail reason
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Error {
        pub error_type: Type,
        pub reason: String,
    }

    impl Error {
        pub fn new(error_type: Type, reason: String) -> Self {
            Error { error_type, reason }
        }
    }

    ///Response struct for returning to the http response.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Accept {
        article_id: String,
        message: String,
    }

    impl Accept {
        pub fn new(article_id: String,reason: String) -> Self {
            Accept { article_id,message: reason }
        }
    }
}
