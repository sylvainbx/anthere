/// This module contains public routes (i.e. routes that can be accessed without prior auth)

use axum::{extract, Router, routing::{get, post}};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_csrf::CsrfToken;
use tower_sessions::Session;
use crate::auth::{AuthSession, Credentials};

pub fn router() -> Router<()> {
    Router::new()
        .route("/", get(home))
        .route("/login", post(login))
}

async fn home(token: CsrfToken, session: Session) -> impl IntoResponse {
    let authenticity_token = token.authenticity_token().unwrap();
    match session.insert("authenticity_token", authenticity_token).await {
        Ok(_) => token.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    };
}

async fn login(
    mut auth_session: AuthSession,
    extract::Json(creds): extract::Json<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return StatusCode::FORBIDDEN.into_response();
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    StatusCode::OK.into_response()
}