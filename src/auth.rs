use axum_login::{AuthnBackend, UserId};
use async_trait::async_trait;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Deserialize;
use crate::errors::{adapt_app_error, AppError};
use crate::models::User;

#[derive(Clone)]
pub struct Backend {
    db: Pool<ConnectionManager<PgConnection>>,
}

impl Backend {
    pub fn new(db: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { db }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
    pub next: Option<String>
}

#[async_trait]
impl AuthnBackend for Backend {
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

pub type AuthSession = axum_login::AuthSession<Backend>;

#[cfg(test)]
mod tests {
    use futures::FutureExt;
    use super::*;
    use crate::{Config, db::TestDb};
    use crate::db::seeds;
    use crate::models::NewUser;

    fn get_test_db() -> TestDb {
        let config = Config::new().unwrap();
        TestDb::new(&config)
    }

    #[tokio::test]
    async fn test_authenticate() {
        let db = get_test_db();
        db.run_test(|pool| async move {
            let seed_users = seeds::users::users();
            let user = seed_users.first().unwrap();

            let backend = Backend::new(pool);
            let creds = Credentials {
                email: user.email.to_string(),
                password: "passw0rd".to_string(),
                next: None
            };
            let res = backend.authenticate(creds).await.unwrap();
            assert!(res.is_some());

            let res = res.unwrap();
            assert_eq!(res.email, user.email);
        }.boxed()).await;
    }

    #[tokio::test]
    async fn test_get_user() {
        let db = get_test_db();
        db.run_test(|pool| async move {
            use crate::schema::users::dsl::*;
            use diesel::prelude::*;

            let conn = &mut pool.get().unwrap();
            let user = NewUser {
                email: "john.doe@eexample.com",
                password: password_auth::generate_hash("passw0rd")
            };
            let user: User = diesel::insert_into(users)
                .values(&user)
                .get_result(conn)
                .unwrap();

            let backend = Backend::new(pool);
            let res = backend.get_user(&user.id).await;
            assert!(res.is_ok());
        }.boxed()).await;
    }
}