CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

-- users (id) {
--         id -> Int4,
--         username -> Varchar,
--         password -> Varchar,
-- }

CREATE TABLE shopping_list_users (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    shopping_list_id INTEGER NOT NULL
);

-- shopping_list_users (id) {
--     id -> Int4,
--     shopping_list_id -> Int4,
--     user_id -> Int4,
-- }
