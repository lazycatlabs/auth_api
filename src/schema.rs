// @generated automatically by Diesel CLI.

diesel::table! {
    login_history (id) {
        id -> Uuid,
        user_id -> Uuid,
        login_timestamp -> Timestamp,
        ip_addr -> Varchar,
        device_info -> Varchar,
        os_info -> Varchar,
        fcm_token -> Varchar,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(login_history -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    login_history,
    users,
);
