-- nanoservices/to_do/dal/migrations/20240628025045_adding-user-connection.sql
CREATE TABLE user_connections (
    user_id INTEGER NOT NULL,
    to_do_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, to_do_id)
);
