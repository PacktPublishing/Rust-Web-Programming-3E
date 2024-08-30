-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    unique_id VARCHAR NOT NULL UNIQUE
);
