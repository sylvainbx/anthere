use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod models;
pub mod schema;

mod config;
pub use config::Config;

pub fn establish_connection(config: &Config) -> PgConnection {
    PgConnection::establish(&config.database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &config.database_url))
}
