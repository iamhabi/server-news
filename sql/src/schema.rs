// @generated automatically by Diesel CLI.

diesel::table! {
    news (id) {
        id -> Int4,
        title -> Text,
        link -> Text,
    }
}