-- Your SQL goes here
CREATE TABLE prices (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    CHECK (name <> '')
);
CREATE TABLE prices_products (

);