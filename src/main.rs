use actix_web::middleware::Logger;
use actix_web::HttpRequest;
use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use article::repository::*;
use article::service::*;
use article::Article;
use std::collections::HashMap;
use std::sync::Mutex;

mod article;
mod configurations;

//TODO FUTURE change api for only access data from json not url
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = configurations::get_app_config();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server_address = format!("{}:{}", app_config.host.clone(), app_config.port.clone());

    println!("-Article service started at address {}.", server_address);

    let repo = ArticleRepository::new(
        app_config.db_config.host.clone(),
        app_config.db_config.port.clone(),
        app_config.db_config.db.clone(),
        app_config.db_config.collection.clone(),
    );
    let service = ArticleService::new(repo);
    //TODO check db status here

    let data = Data::new(Mutex::new(service));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(create_article)
            .service(find)
            .service(list)
            .wrap(Logger::default())
    })
    .bind(&server_address)
    .unwrap_or_else(|err| {
        panic!(
            "Couldnt start the server at address: {}, Error-> {:?}",
            &server_address, err
        )
    })
    .run()
    .await
}

#[get("/api/get")]
async fn find(request: HttpRequest, repo: Data<Mutex<ArticleService>>) -> impl Responder {
    let service = repo.lock().unwrap();
    let params = web::Query::<HashMap<String, String>>::from_query(request.query_string());
    match params {
        Ok(query) => {
            let id = match query.get("id") {
                Some(e) => e,
                None => return HttpResponse::BadRequest().json("No ID param found"),
            };
            let result = service.get_article(id.to_string()).await;
            match result {
                //FIXME refactor error response
                Ok(accept) => {
                    if let Some(article) = accept {
                        HttpResponse::Ok().json(article)
                    } else {
                        HttpResponse::BadRequest().json("Article not found")
                    }
                }
                Err(err) => format_error(err),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

#[post("/api/createArticle")]
async fn create_article(
    article: web::Json<Article>,
    repo: Data<Mutex<ArticleService>>,
) -> impl Responder {
    let service = repo.lock().unwrap();
    let result = service.insert_article(article.clone()).await;
    match result {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(err) => format_error(err),
    }
}
#[get("/api/list")]
async fn list(request: HttpRequest, repo: Data<Mutex<ArticleService>>) -> impl Responder {
    let service = repo.lock().unwrap();

    let params = web::Query::<HashMap<String, String>>::from_query(request.query_string());
    match params {
        Ok(query)=>{
            let size = match query.get("size") {
                Some(e) => match e.parse::<i64>(){
                    Ok(res)=>res,
                    Err(_)=> return HttpResponse::BadRequest().json("The size param must be a number")
                },
                None => return HttpResponse::BadRequest().json("No size param found"),
            };
            let result = service.list(size).await;
            match result {
                Ok(vector) => HttpResponse::Ok().json(vector),
                Err(err) => format_error(err),
            }
        },
        Err(e)=> HttpResponse::BadRequest().json(e.to_string())
    }
    
}

fn format_error(error: mongodb::error::Error) -> HttpResponse {
    //TODO refactor clearly
    HttpResponse::BadRequest().json(error.to_string())
}
