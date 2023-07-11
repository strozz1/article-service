use actix_web::middleware::Logger;
use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use article::repository::*;
use article::service::*;
use article::Article;
use request::Request;
use response::error::Error;
use response::*;
use std::sync::Mutex;

mod article;
mod request;
mod response;

//TODO FUTURE change api for only access data from json not url
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //config
    dotenv::dotenv().ok();
    let server_port = std::env::var("PORT").unwrap();
    let server_address = format!("localhost:{}", server_port);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_host = std::env::var("DB_HOST").unwrap();
    let db_port: u16 = std::env::var("DB_PORT").unwrap().parse().unwrap();

    println!("-Article service started at address {}.", server_address);

    let repo = ArticleRepository::new(db_host, db_port);
    let service = ArticleService::new(repo);
    //TODO check db status here

    let data = Data::new(Mutex::new(service));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(root)
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
#[get("/api")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("root directory")
}
#[get("/api/find")]
async fn find(
    article_id: web::Json<Request<String>>,
    repo: Data<Mutex<ArticleService>>,
) -> impl Responder {
    let service = repo.lock().unwrap(); // get repo from server
    let result = service.get_article(article_id.content.clone()).await;
    match result {
        //FIXME refactor error response
        Ok(accept) => HttpResponse::Ok().json(Response::new(RequestStatus::Ok, accept)),
        Err(err) => format_error(err),
    }
}

#[post("/api/createArticle")]
async fn create_article(
    json: web::Json<Request<Article>>,
    repo: Data<Mutex<ArticleService>>,
) -> impl Responder {
    let service = repo.lock().unwrap(); // get repo from server
    let result = service.insert_article(json.content.clone()).await;
    match result {
        Ok(accept) => HttpResponse::Ok().json(Response::new(RequestStatus::Ok, accept)),
        Err(err) => format_error(err),
    }
}
#[get("/api/list")]
async fn list(size: web::Json<Request<i64>>, repo: Data<Mutex<ArticleService>>) -> impl Responder {
    let service = repo.lock().unwrap(); // get repo from server
    let result = service.list(size.content).await;
    match result {
        Ok(vector) => {
            HttpResponse::Ok().json(Response::new_from_multiple(RequestStatus::Ok, vector))
        }
        Err(err) => format_error(err),
    }
}

fn format_error(error: Error) -> HttpResponse {
    //TODO refactor clearly
    match error.code {
        0 => HttpResponse::NotAcceptable().json(Response::new(RequestStatus::InvalidId, error)),
        1 => HttpResponse::NotFound().json(Response::new(RequestStatus::NotFound, error)),
        2 => HttpResponse::BadRequest().json(Response::new(RequestStatus::Database, error)),
        3 => {
            HttpResponse::InternalServerError().json(Response::new(RequestStatus::Internal, error))
        }
        4 => HttpResponse::BadRequest().json(Response::new(RequestStatus::DuplicateKey, error)),
        5 => HttpResponse::RequestTimeout().json(Response::new(RequestStatus::Timeout, error)),
        _ => {
            HttpResponse::InternalServerError().json(Response::new(RequestStatus::Internal, error))
        }
    }
}
