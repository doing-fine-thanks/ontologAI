use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use askama_actix::TemplateToResponse;
use env_logger;
use lazy_static::lazy_static;
use log::info;
use std::env;

use crate::chat::create_ontological_representation;
use crate::requests::SearchTermRequest;
use crate::templates::IndexTemplate;
use crate::utils::{interal_error, not_found};

pub mod chat;
pub mod requests;
pub mod templates;
pub mod utils;

lazy_static! {
    static ref OPEN_API_KEY: String = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(_) => panic!("No enivroment variable OPENAI_API_KEY found!"),
    };
    static ref DB_URL: String = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => panic!("No enivroment variable DATABASE_URL found!"),
    };
}

async fn show_ontology(terms_request: &SearchTermRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let result = create_ontological_representation(&terms_request.search_term).await.expect("error displaying ontology");
    Ok(result.to_response())
}

#[post("/show")]
async fn show(search_term: web::Json<SearchTermRequest>) -> impl Responder {
    let result = show_ontology(&search_term.into_inner()).await;
    match result {
        Ok(result) => result,
        Err(_) => interal_error(),
    }
}

#[post("/echo")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Here!")
}

#[get("/")]
async fn index() -> impl Responder {
    IndexTemplate {}.to_response()
}

async fn default_not_found() -> impl Responder {
    not_found()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new());
    let addr = "127.0.0.1";
    let port = 8080;

    info!(" 🌐 will server on: http://{}:{} 🌐 ", addr, port);
    HttpServer::new(move || {
        App::new()
            .service(echo)
            .service(index)
            .service(show)
            .default_service(web::to(default_not_found))
            .wrap(Logger::default())
    })
    .bind((addr, port))?
    .run()
    .await
}
