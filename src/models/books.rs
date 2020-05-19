use crate::errors::errors::{AppError, AppErrorType::*};
use std::io;

use serde::{Serialize, Deserialize};

use deadpool_postgres::Client;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_pg_mapper::FromTokioPostgresRow;


#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="books")]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub have_read: bool
}

#[derive(Serialize, Deserialize)]
pub struct NewBook{
    pub title: String,
    pub author: String,
}

pub async fn get_books_from_db(client: &Client) -> Result<Vec<Book>, io::Error>{
    let statement = client.prepare("select * from books order by id desc").await.unwrap();

    let books = client.query(&statement, &[])
        .await
        .expect("Error getting books")
        .iter()
        // map the sql row to the struct
        .map(|row| Book::from_row_ref(row).unwrap())
        .collect::<Vec<Book>>();

    Ok(books)
}

pub async fn get_book_by_id(client: &Client, id: i32) -> Result<Book, AppError> {
    let statement = client
        .prepare("select * from book where id = $1")
        .await?;

    let book = client
        .query_opt(&statement, &[&id])
        .await?
        .map(|row| Book::from_row_ref(&row).unwrap());

    match book {
        Some(book) => Ok(book),
        None => Err(AppError {
            message: Some("Error getting Book".to_string()),
            cause: Some("Unknown error.".to_string()),
            error_type: DbError,
        }),
    }
}


pub async fn create_book_in_db(client: &Client, title: String, author: String) -> Result<Book, AppError>{
    let statement = client.prepare("insert into books {title} {author} values ($1) ($2) returning id").await.unwrap();

    client.query(&statement, &[&title, &author])
        .await
        .expect("Error creating book")
        .iter()
        .map(|row| Book::from_row_ref(row).unwrap())
        .collect::<Vec<Book>>()
        // return value via pop or error
        .pop()
        .ok_or(AppError {
            message: Some("Error creating Book".to_string()),
            cause: Some("Unknown error.".to_string()),
            error_type: DbError,
        })
}