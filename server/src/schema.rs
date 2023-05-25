// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Integer,
        user_id -> Integer,
        session_id -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        pass -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
