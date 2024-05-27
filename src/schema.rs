// @generated automatically by Diesel CLI.

diesel::table! {
    to_do (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        status -> Varchar,
        date -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        admin -> Int4,
    }
}

diesel::joinable!(to_do -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    to_do,
    users,
);
