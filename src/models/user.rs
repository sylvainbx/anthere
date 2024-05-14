use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub reset_password_token: Option<String>,
    pub reset_password_sent_at: Option<chrono::NaiveDateTime>,
    pub sign_in_count: i32,
    pub current_sign_in_at: Option<chrono::NaiveDateTime>,
    pub last_sign_in_at: Option<chrono::NaiveDateTime>,
    pub current_sign_in_ip: Option<ipnet::IpNet>,
    pub last_sign_in_ip: Option<ipnet::IpNet>,
    pub confirmation_token: Option<String>,
    pub confirmation_sent_at: Option<chrono::NaiveDateTime>,
    pub confirmed_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}