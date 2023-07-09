use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

pub mod repository;
pub mod service;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub id: String,
    pub author: String,
    pub create_date: DateTime,
    pub update_date: DateTime,
    pub content: String,
}
