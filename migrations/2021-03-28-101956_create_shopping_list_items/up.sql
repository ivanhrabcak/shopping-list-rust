CREATE TABLE shooping_list_items (
    id INTEGER NOT NULL PRIMARY KEY,
    shooping_list_id INTEGER NOT NULL,
    item_name VARCHAR,
    done BOOLEAN NOT NULL DEFAULT 'f'
);

    -- shopping_list_items (id) {
    --     id -> Int4,
    --     shopping_list_id -> Int4,
    --     item_name -> Nullable<Varchar>,
    --     done -> Bool,
    -- }