// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Integer,
        owner_id -> Integer,
        account_number -> Text,
        balance -> Text,
        active -> Integer,
    }
}

diesel::table! {
    actions (id) {
        id -> Integer,
        created_at -> Text,
        from_acc -> Text,
        to_acc -> Text,
        amount -> Text,
    }
}

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
    accounts,
    actions,
    sessions,
    users,
);
