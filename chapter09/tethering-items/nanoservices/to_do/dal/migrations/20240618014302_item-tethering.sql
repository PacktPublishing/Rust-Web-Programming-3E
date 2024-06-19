-- Add migration script here
CREATE TABLE user_connections (
    user_id INTEGER NOT NULL,
    to_do_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, to_do_id),
);
