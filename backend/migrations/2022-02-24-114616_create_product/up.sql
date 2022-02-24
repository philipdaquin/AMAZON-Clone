-- Your SQL goes here
CREATE TABLE products ( 
    id    SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    stock FLOAT NOT NULL,
    rating INTEGER,
    price INTEGER
)