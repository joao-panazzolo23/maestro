// @generated automatically by Diesel CLI.

diesel::table! {
    behaviors (id) {
        id -> Integer,
        display_name -> Text,
        content -> Text,
        is_active -> Bool,
    }
}
