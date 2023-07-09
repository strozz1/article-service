use serde::{Serialize, Deserialize};

 ///Response struct for returning to the http response.
 #[derive(Serialize, Deserialize, Debug)]
 pub struct Accept {
     article_id: String,
     message: String,
 }

 impl Accept {
     pub fn new(article_id: String, reason: String) -> Self {
         Accept {
             article_id,
             message: reason,
         }
     }
 }