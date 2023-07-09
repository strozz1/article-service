use std::{sync::{Mutex}};
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web::{self, Data}, post};


use crate::{article::{Article, ArticleRepository}, service::service::ArticleService, response::response::{Type, Response}};
mod response;
mod article;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //config
    dotenv::dotenv().ok();
    let server_port = std::env::var("PORT").unwrap_or("2425".to_string());
    let server_address = format!("localhost:{}", server_port);

    let db_host = std::env::var("DB_HOST").unwrap_or("localhost".to_string());
    let db_port: u16 = std::env::var("DB_PORT").unwrap_or("27018".to_string()).parse().unwrap();
    
    println!("-Article service started at address {}.", server_address);

    let repo=ArticleRepository::new(db_host, db_port);
    let service= ArticleService::new(repo);

    let data =  Data::new(Mutex::new(service));
    
    HttpServer::new(move|| {
        App::new()
        .app_data(Data::clone(&data))
            .service(root)
            .service(list)
            .service(create_article)
            .service(web::resource("/{id}").route(web::get().to(get_by_id)))
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
#[get("/")]
async fn root() -> impl Responder {
 
    HttpResponse::Ok().body("root directory")
}

#[get("/list")]
async fn list() -> impl Responder {
    HttpResponse::Ok().body("list")
}

async fn get_by_id(article_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("id: {}",article_id))
}


#[post("/createArticle")]
async fn create_article(json: web::Json<Article>,repo: Data<Mutex<ArticleService>>) -> impl Responder{
    let service = repo.lock().unwrap(); // get repo from server
    let result = service.insert_article(json.0).await;
    match result {
        Ok(accept) => HttpResponse::Ok().json(Response::new(Type::Ok, accept)),
        Err(err) => match err.error_type{
            Type::BadRequest =>  HttpResponse::BadRequest().json(Response::new(Type::BadRequest, err)),
            Type::MalformedJSON => HttpResponse::PreconditionRequired().json(Response::new(Type::MalformedJSON, err)),
            Type::Database => HttpResponse::NotAcceptable().json(Response::new(Type::Database, err)),
            Type::Internal => HttpResponse::InternalServerError().json(Response::new(Type::Internal, err)),
            Type::Ok => HttpResponse::Ok().json(Response::new(Type::Ok, err)),
        }
    }

}