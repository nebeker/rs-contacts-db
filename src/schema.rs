// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        active -> Bool,
    }
}
