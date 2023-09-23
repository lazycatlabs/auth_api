// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
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
