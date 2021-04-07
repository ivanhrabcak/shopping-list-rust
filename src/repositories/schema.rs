table! {
    shopping_list_items (id) {
        id -> Int4,
        shopping_list_id -> Int4,
        item_name -> Nullable<Varchar>,
        done -> Bool,
    }
}

table! {
    shopping_list_users (id) {
        id -> Int4,
        shopping_list_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    shopping_lists (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    shopping_list_items,
    shopping_list_users,
    shopping_lists,
    users,
);
