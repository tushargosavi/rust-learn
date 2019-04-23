table! {
    heroes (id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    heroes,
    posts,
);
