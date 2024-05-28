use diesel::prelude::*;
use axum_login::AuthUser;

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
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

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("password_hash", &"[redacted]")
            .field("reset_password_token", &"[redacted]")
            .field("reset_password_sent_at", &self.reset_password_sent_at)
            .field("sign_in_count", &self.sign_in_count)
            .field("current_sign_in_at", &self.current_sign_in_at)
            .field("last_sign_in_at", &self.last_sign_in_at)
            .field("current_sign_in_ip", &self.current_sign_in_ip)
            .field("last_sign_in_ip", &self.last_sign_in_ip)
            .field("confirmation_token", &self.confirmation_token)
            .field("confirmation_sent_at", &self.confirmation_sent_at)
            .field("confirmed_at", &self.confirmed_at)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .finish()
    }
}

impl AuthUser for User {
    type Id = i32;
    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password.as_bytes()
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: String,
}
