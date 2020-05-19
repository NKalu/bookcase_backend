use crate::handlers::book_handlers::get_books;
use actix_web::web;

pub fn books_config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::resource("/all_books")
            .route(web::get().to(get_books)),
    );
}