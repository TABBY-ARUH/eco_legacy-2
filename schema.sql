// src/schema.rs
table! {
    projects (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        image_url -> Nullable<Text>,
        multimedia_url -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        hashed_password -> Text,
    }
}
