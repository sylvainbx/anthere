use axum_login::AuthManagerLayerBuilder;
use tower_sessions::cookie::Key;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::r2d2::{ConnectionManager, Pool};
use tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer};
use crate::{Config, get_connection_pool};
use crate::auth::Backend;
use crate::errors::adapt_app_error;
use crate::store::PgStore;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations/");

pub mod routes;

pub struct App {
    db: Pool<ConnectionManager<PgConnection>>,
    config: Config
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {

        let config = Config::new().map_err(adapt_app_error)?;
        let db = get_connection_pool(&config.database_url);
        
        let mut conn = db.get().map_err(adapt_app_error)?;
        conn.run_pending_migrations(MIGRATIONS).map_err(adapt_app_error)?;

        Ok(App { db, config })
    }
    
    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        // handle the session
        let session_store = PgStore::new(self.db.clone());
        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        // Generate a cryptographic key to sign the session cookie.
        let key = Key::generate();

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(time::Duration::days(1)))
            .with_signed(key);
        
        // handle the authentication
        let backend = Backend::new(self.db.clone());
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();
        
        //TODO
        // let app = protected::router()
        //     .route_layer(login_required!(Backend, login_url = "/login"))
        //     .merge(auth::router())
        //     .layer(MessagesManagerLayer)
        //     .layer(auth_layer);
        
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        
        tracing::debug!("listening on {}", listener.local_addr()?);
        axum::serve(listener, routes::router()).await?;
        
        deletion_task.await??;
        
        Ok(())
    }
}
