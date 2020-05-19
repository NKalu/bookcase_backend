mod config;
mod models;
mod handlers;
mod routes;

use actix_web::{web, middleware,
    HttpServer, App};
use std::io;
use env_logger::Env;
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()>{

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting bookcase_backend at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new(move || {

        App::new()
            // Logging middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            // Normalizes path of request
            .wrap(middleware::NormalizePath)
            // compression middleware for file handling
            // .wrap(middleware::Compress::default())
            // setting up db pool for apps
            .data(pool.clone())
            .configure(routes::book_routes::books_config)
            //{_:/?} is regex to match to trailing slash
            .route("/", web::get().to(handlers::book_handlers::welcome))
        })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run().
        await
    }