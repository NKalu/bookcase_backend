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

pub async fn create_book_in_db(client: &Client, title: String, author: String) -> Result<Book, io::Error>{
    let statement = client.prepare("insert into books {title} {author} values ($1) ($2) returning id").await.unwrap();

    client.query(&statement, &[&title, &author])
        .await
        .expect("Error creating book list")
        .iter()
        .map(|row| Book::from_row_ref(row).unwrap())
        .collect::<Vec<Book>>()
        // return value via pop or error
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating book"))
}