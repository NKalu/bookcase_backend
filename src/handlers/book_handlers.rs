use crate::models::books::{
    Book, 
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