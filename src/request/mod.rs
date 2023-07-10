use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Request<T>{
    pub content: T
}