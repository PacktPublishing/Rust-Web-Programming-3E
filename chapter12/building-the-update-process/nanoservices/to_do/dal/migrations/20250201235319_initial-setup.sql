-- Add migration script here
CREATE TABLE to_do_items (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) UNIQUE NOT NULL,
    status VARCHAR(7) NOT NULL
);
