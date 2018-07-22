table! {
    administrators (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        level -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    developers (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    games (id) {
        id -> Int4,
        developer_id -> Int4,
        name -> Varchar,
        url -> Varchar,
        state -> Int2,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(games -> developers (developer_id));

allow_tables_to_appear_in_same_query!(
    administrators,
    developers,
    games,
    users,
);
