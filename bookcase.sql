drop table if EXISTS books;

CREATE TABLE books (
    id serial PRIMARY KEY,
    title VARCHAR NOTNULL,
    author VARCHAR NOTNULL,
    have_read BOOLEAN NOT NULL DEFAULT 'f'
);
