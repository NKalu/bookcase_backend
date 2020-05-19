use crate::models::books::{
    NewBook,
    create_book_in_db, 
    get_books_from_db
};
use serde::{Serialize, Deserialize};
use deadpool_postgres::{Pool, Client};
use actix_web::{web, HttpResponse, Responder};


#[derive(Serialize, Deserialize)]
pub struct Welcome {
    pub welcome: String
}
pub async fn welcome() -> impl Responder {
    //check if server is running
    web::HttpResponse::Ok()
        .json( Welcome { welcome: "Welcome to your bookcase".to_string() })
}
                

pub async fn get_books(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = get_books_from_db(&client).await;

    match result {
        Ok(books) => HttpResponse::Ok().header("X-Total-Count", books.len()).json(books),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn create_book(db_pool: web::Data<Pool>, json: web::Json<NewBook>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = create_book_in_db(&client, json.title.clone(), json.author.clone()).await;

    match result {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}