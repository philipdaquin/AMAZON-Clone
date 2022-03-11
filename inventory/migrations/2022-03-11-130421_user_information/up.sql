-- Your SQL goes here
CREATE TABLE users (
    email VARCHAR(100) NOT NULL PRIMARY KEY,
    password VARCHAR(64) NOT NULL,
    created_at TIMESTAMP NOT NULL
);
