use std::time::Duration;

use async_trait::async_trait;
use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOneOptions, FindOptions, ServerAddress},
    Client, Collection,
};
use serde::Serialize;

use super::Article;

#[derive(Clone)]
pub struct ArticleRepository {
    client: Client,
    db: String,
    collection: String,
}

impl ArticleRepository {
    pub fn new(host: String, port: u16,db:String,collection:String) -> Self {

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
            db,
            collection
        }
    }
}

#[async_trait]
pub trait Repository<T: Serialize> {
    async fn insert(&self, data: T) -> Result<String, mongodb::error::Error>;
    async fn check_status(&self) -> bool;
    async fn find(&self, id: String) -> Result<Option<Article>, mongodb::error::Error>;
    async fn list(&self, size: i64) -> Result<Vec<Article>, mongodb::error::Error>;
}

#[async_trait]
impl Repository<Article> for ArticleRepository {
    ///Takes an article as an argument and inserts the article into the database.
    ///This method does not check the article status(correct id, empty data...)
    ///
    /// Returns the article id if the article has been sucesfully added to the database
    async fn insert(&self, article: Article) -> Result<String, mongodb::error::Error> {
        //todo time out
        let collection: Collection<Article> = self
            .client
            .database(&self.db)
            .collection::<Article>(&self.collection);

        collection.insert_one(article.clone(), None).await?;
        Ok(article.id)
    }

    async fn check_status(&self) -> bool {
        // self.client.
        todo!()
    }

    async fn find(&self, article_id: String) -> Result<Option<Article>, mongodb::error::Error> {
        let collection: Collection<Article> = self
            .client
            .database(&self.db)
            .collection::<Article>(&self.collection);

        let find_options = FindOneOptions::builder()
            .projection(doc! {"_id": 0 })
            .max_time(Some(Duration::from_secs(3)))
            .build();

        let article = collection
            .find_one(doc! {"id": article_id}, Some(find_options))
            .await?;
        Ok(article)
    }

    async fn list(&self, size: i64) -> Result<Vec<Article>, mongodb::error::Error> {
        let collection: Collection<Article> = self
            .client
            .database(&self.db)
            .collection::<Article>(&self.collection);

        let find_options = FindOptions::builder()
            .projection(doc! {"_id": 0 })
            .max_time(Some(Duration::from_secs(3)))
            .max_await_time(Some(Duration::from_secs(3)))
            .limit(Some(size))
            .build();

        let mut cursor = collection.find(doc! {}, Some(find_options)).await?;

        let mut vector: Vec<Article> = vec![];
        while let Ok(next) = cursor.advance().await {
            if next {
                let deserialized = cursor.deserialize_current();
                match deserialized {
                    Ok(article) => vector.push(article),
                    Err(_) => (),
                }
            } else {
                break;
            }
        }
        Ok(vector)
    }
}
