use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::r2d2::{ConnectionManager, Pool};
use crate::{Config, get_connection_pool};
use crate::errors::adapt_app_error;


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub mod routes;

pub struct Server {
    db: Pool<ConnectionManager<PgConnection>>,
    config: Config
}

impl Server {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {

        let config = Config::new().map_err(adapt_app_error)?;
        let db = get_connection_pool(&config);
        
        let mut conn = db.get().map_err(adapt_app_error)?;
        conn.run_pending_migrations(MIGRATIONS).map_err(adapt_app_error)?;

        Ok(Server { db, config })
    }
    
    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        
        tracing::debug!("listening on {}", listener.local_addr()?);
        axum::serve(listener, routes::router()).await?;
        
        Ok(())
    }
}
