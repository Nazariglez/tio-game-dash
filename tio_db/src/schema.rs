table! {
    administrator_sessions (id) {
        id -> Int4,
        administrator_id -> Int4,
        is_valid -> Bool,
        expire_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
    apps (id) {
        id -> Int4,
        developer_id -> Nullable<Int4>,
        name -> Varchar,
        url -> Varchar,
        state -> Int2,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    apps_categories (id) {
        id -> Int4,
        app_id -> Int4,
        category_id -> Int4,
    }
}

table! {
    apps_tags (id) {
        id -> Int4,
        app_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    developer_sessions (id) {
        id -> Int4,
        developer_id -> Int4,
        is_valid -> Bool,
        expire_at -> Timestamp,
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
    keys (id) {
        id -> Int4,
        app_id -> Int4,
        live -> Bool,
        public -> Bpchar,
        secret -> Bpchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_sessions (id) {
        id -> Int4,
        user_id -> Int4,
        is_valid -> Bool,
        expire_at -> Timestamp,
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

joinable!(administrator_sessions -> administrators (administrator_id));
joinable!(apps -> developers (developer_id));
joinable!(apps_categories -> apps (app_id));
joinable!(apps_categories -> categories (category_id));
joinable!(apps_tags -> apps (app_id));
joinable!(apps_tags -> tags (tag_id));
joinable!(developer_sessions -> developers (developer_id));
joinable!(keys -> apps (app_id));
joinable!(user_sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    administrator_sessions,
    administrators,
    apps,
    apps_categories,
    apps_tags,
    categories,
    developer_sessions,
    developers,
    keys,
    tags,
    user_sessions,
    users,
);
