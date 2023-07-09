use std::time::Duration;

use async_trait::async_trait;
use mongodb::{
    bson::doc,
    error,
    options::{ClientOptions, ServerAddress},
    results::InsertOneResult,
    Client, Collection,
};
use serde::{Deserialize, Serialize};

use crate::response::{response::Accept, response::{Error, Type}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub id: String,
    pub author: String,
    pub create_date: String,
    pub update_date: Option<String>,
    pub content: String,
}

#[derive(Clone)]
pub struct ArticleRepository {
    client: Client,
    db_name: String,
    db_collection: String,
}

impl ArticleRepository {
    pub fn new(host: String, port: u16) -> Self {
        let db_name = std::env::var("DB_NAME").unwrap_or("test".to_string());
        let db_collection = std::env::var("DB_COLLECTION").unwrap_or("articles".to_string());

        let client_options = ClientOptions::builder()
            .connect_timeout(Some(Duration::from_secs(1)))
            .hosts([ServerAddress::Tcp {
                host,
                port: Some(port),
            }])
            .build();
        let client = Client::with_options(client_options);
        let client = client.unwrap();
        ArticleRepository {
            client,
            db_name,
            db_collection,
        }
    }
}

#[async_trait]
pub trait Repository<T: Serialize> {
    async fn insert(&self, data: T) -> Result<Accept, Error>;
   // async fn find(&self, id: String) -> Result<Article, String>;
   async fn check_status(&self) -> bool;
}


#[async_trait]
impl Repository<Article> for ArticleRepository {
    ///Takes an article as an argument and inserts the article into the database.
    ///This method does not check the article status(correct id, empty data...)
    ///
    /// Returns if the article has been sucesfully added to the database
    async fn insert(&self, article: Article) -> Result<Accept, Error> {
        //todo time out
        let collection: Collection<Article> = self
            .client
            .database(&self.db_name)
            .collection::<Article>(&self.db_collection);
        let response = article.clone();

        let handle: tokio::task::JoinHandle<Result<InsertOneResult, error::Error>> =
            tokio::spawn(async move { collection.insert_one(article, None).await });

        let result: Result<Result<InsertOneResult, error::Error>, tokio::task::JoinError> = handle.await;
        match result {
            Ok(result) => {
                match result {
                    Ok(_) => Ok(Accept::new(response.id, "Article saved".to_string())),
                    Err(internalerr) => Err(Error::new(Type::Database,internalerr.to_string())),
                }
            },
            Err(err) => Err(Error::new(Type::Internal, err.to_string())), // error en tokio
        }
    }
    // async fn find(&self, article_id: String) -> Result<Article, String> {
    //     let collection: Collection<Article> = self
    //         .client
    //         .database(&self.db_name)
    //         .collection::<Article>(&self.db_collection);

    //     collection.find_one(doc! {"id": article_id}, None).await;
    // }
    async fn check_status(&self) -> bool{
        // self.client.
        todo!()
    }
}
