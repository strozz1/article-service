pub mod service {

    use crate::article::Article;
    use crate::article::ArticleRepository;
    use crate::article::Repository;
    use crate::response::response::Accept;
    use crate::response::response::Error;

    pub struct ArticleService {
        pub repository: ArticleRepository,
    }

    impl ArticleService {
        pub fn new(repository: ArticleRepository) -> Self {
            ArticleService { repository }
        }

        pub async fn insert_article(&self, article: Article) -> Result<Accept,Error> {
            //todo check for article format
            
             self.repository.insert(article).await
        }

        pub async fn check_status(&self){
            self.repository.check_status();
        }

        ///Returns an article from the repository with the same ID as given
        pub async fn get_article(&self, id: String) -> Result<Article,Error>{
            self.repository.find(id).await
        }
    }
}
