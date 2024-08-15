use axum_csrf::CsrfToken;
use tower_sessions_core::Session;
use axum::Form;

pub struct CsrfProtected {
    authenticity_token: String
}

pub async fn check_key(token: CsrfToken, session: Session, Form(payload): Form<CsrfProtected>, callback: Box<dyn Fn()>) {
    let authenticity_token: Option<String> = session.get("authenticity_token").await.unwrap_or_default();

    match authenticity_token {
        None => { tracing::warn!("Token was not set"); },
        Some(authenticity_token) => {
            if let Err(_) = token.verify(&payload.authenticity_token) {
                tracing::warn!("Token is invalid");
            } else if let Err(_) = token.verify(&authenticity_token) {
                tracing::warn!("Modification of both Cookie/token OR a replay attack occurred");
            } else {
                // we remove it to only allow one post per generated token.
                session.remove::<String>("authenticity_token").await.unwrap();
                callback();
            }
        }
    }
}