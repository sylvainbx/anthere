use futures::future::BoxFuture;
use diesel::{sql_query, Connection, PgConnection, RunQueryDsl};
use std::sync::atomic::AtomicU32;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::Pool;
use url::Url;
use crate::{Config, get_connection_pool};
use crate::db::seeds::seeds;
use std::thread;

static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations/");

pub struct TestDb {
    default_db_url: String,
    name: String,
    pool: Pool<ConnectionManager<PgConnection>>,
}
impl TestDb {
    pub fn new(config: &Config) -> Self {
        let name = format!(
            "test_db_{}_{}",
            std::process::id(),
            TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );
        let default_db_url = &config.database_url;
        let mut conn = PgConnection::establish(&default_db_url).unwrap();

        sql_query(format!("CREATE DATABASE {};", name))
            .execute(&mut conn)
            .unwrap();
        let mut url = Url::parse(&default_db_url).unwrap();
        url.set_path(&name);

        Self {
            default_db_url: default_db_url.to_string(),
            name,
            pool: get_connection_pool(&url.to_string()),
        }
    }

    pub async fn run_test(&self, test: impl Fn(Pool<ConnectionManager<PgConnection>>) -> BoxFuture<'static, ()>) {
        let conn = &mut self.pool.get()
            .expect("Unable to connect to the test database");

        conn.run_pending_migrations(MIGRATIONS)
            .expect("Unable to migrate the test database");

        seeds(conn).expect("Unable to seed the test database");

        test(self.pool.clone()).await;
    }
}
impl Drop for TestDb {
    fn drop(&mut self) {
        if thread::panicking() {
            eprintln!("TestDb leaking database {}", self.name);
            return;
        }
        let mut conn = PgConnection::establish(&self.default_db_url).unwrap();
        sql_query(format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
            self.name
        )).execute(&mut conn)
            .unwrap();
        sql_query(format!("DROP DATABASE {}", self.name))
            .execute(&mut conn)
            .unwrap();
    }
}