// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password_hash -> Bytea,
        reset_password_token -> Nullable<Varchar>,
        reset_password_sent_at -> Nullable<Timestamp>,
        sign_in_count -> Int4,
        current_sign_in_at -> Nullable<Timestamp>,
        last_sign_in_at -> Nullable<Timestamp>,
        current_sign_in_ip -> Nullable<Inet>,
        last_sign_in_ip -> Nullable<Inet>,
        confirmation_token -> Nullable<Varchar>,
        confirmation_sent_at -> Nullable<Timestamp>,
        confirmed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
