// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Integer,
        user_name -> Text,
        register_date -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
        poster_id -> Integer,
        create_date -> Timestamp,
        modify_date -> Nullable<Timestamp>,
    }
}

diesel::joinable!(posts -> accounts (poster_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    posts,
);
