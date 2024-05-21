use axum_login::{AuthnBackend, UserId};
use async_trait::async_trait;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Deserialize;
use crate::errors::{adapt_app_error, AppError};
use crate::models::User;

#[derive(Clone)]
pub struct Backend<'a> {
    db: &'a Pool<ConnectionManager<PgConnection>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
    pub next: Option<String>
}

#[async_trait]
impl<'a> AuthnBackend for Backend<'a> {
    type User = User;
    type Credentials = Credentials;
    type Error = AppError;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        
        let conn = &mut self.db.get().map_err(adapt_app_error)?;

        let user = users
            .filter(email.eq(credentials.email))
            .select(User::as_select())
            .limit(1)
            .get_result(conn)
            .optional()
            .map_err(adapt_app_error)?;

        tokio::task::spawn_blocking(|| {
            Ok(user.filter(|user|
                password_auth::verify_password(
                    credentials.password,
                    &String::from(&user.password)
                ).is_ok()))
        }).await
            .map_err(adapt_app_error)?
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;

        let conn = &mut self.db.get().map_err(adapt_app_error)?;

        let user = users.find(user_id)
            .select(User::as_select())
            .first(conn)
            .optional()
            .map_err(adapt_app_error)?;
        
        Ok(user)
    }
}

pub type AuthSession<'a> = axum_login::AuthSession<Backend<'a>>;