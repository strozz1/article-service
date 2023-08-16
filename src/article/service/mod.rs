
use super::repository::ArticleRepository;
use super::repository::*;
use super::Article;

pub struct ArticleService {
    pub repository: ArticleRepository,
}

impl ArticleService {
    pub fn new(repository: ArticleRepository) -> Self {
        ArticleService { repository }
    }

    pub async fn insert_article(&self, article: Article) -> Result<String, mongodb::error::Error> {
        //todo check for article format

        self.repository.insert(article).await
    }

    pub async fn check_status(&self) {
        self.repository.check_status();
        //TODO check database status
    }

    ///Returns an article from the repository with the same ID as given
    pub async fn get_article(&self, id: String) -> Result<Option<Article>, mongodb::error::Error> {
        self.repository.find(id).await
    }

    ///Returns a result  from the repository witha vector of articles or an error
    pub async fn list(&self, size: i64) -> Result<Vec<Article>, mongodb::error::Error> {
        self.repository.list(size).await
    }
}
