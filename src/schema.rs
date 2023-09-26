// @generated automatically by Diesel CLI.

diesel::table! {
    login_history (id) {
        id -> Int4,
        user_id -> Uuid,
        login_timestamp -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        photo -> Varchar,
        verified -> Bool,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        role -> Varchar,
        login_session -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(login_history -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    login_history,
    users,
);
