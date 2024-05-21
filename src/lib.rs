use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub mod models;
pub mod schema;

pub mod errors;

mod config;
pub use config::Config;

mod server;
pub use server::Server;

mod auth;

pub fn get_connection_pool(config: &Config) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
