use crate::handlers::book_handlers::{get_books, create_book, get_book_from_id};
use actix_web::web;

pub fn books_config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::resource("/all_books")
            .route(web::get().to(get_books)),
    );
    cfg.service(
        web::resource("/create_book")
        .route(web::post().to(create_book)),
    );
    cfg.service(
        web::resource("/book/{id}")
        .route(web::get().to(get_book_from_id)),
    );
}