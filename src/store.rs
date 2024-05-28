use async_trait::async_trait;
use diesel::{Connection, ExpressionMethods, PgConnection, RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use tower_sessions::{
    session::{Id, Record},
    session_store, ExpiredDeletion, SessionStore,
};
use crate::models::Session;

#[derive(Clone, Debug)]
pub struct PgStore {
    db: Pool<ConnectionManager<PgConnection>>,
}

impl PgStore {

    pub fn new(db: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { db }
    }
    fn id_exists(&self, conn: &mut PgConnection, session_id: &Id) -> diesel::QueryResult<bool> {
        use crate::schema::sessions::dsl::*;
        use diesel::{select, dsl::exists, prelude::*};

        select(exists(sessions
            .filter(id.eq(session_id.to_string()))
        )).get_result(conn)
    }

    fn save_with_conn(
        &self,
        conn: &mut PgConnection,
        record: &Record,
    ) -> diesel::QueryResult<()> {
        use crate::schema::sessions::dsl::*;
        use diesel::upsert::excluded;

        let new_session = Session {
            id: record.id.to_string(),
            data: rmp_serde::to_vec(&record.data)
                .map_err(adapt_serial_err)?,
            expiry_date: record.expiry_date
        };

        diesel::insert_into(sessions)
            .values(&new_session)
            .on_conflict(id)
            .do_update()
            .set((
                data.eq(excluded(data)),
                expiry_date.eq(excluded(expiry_date))
            ))
            .execute(conn)?;

        Ok(())
    }

}

#[async_trait]
impl ExpiredDeletion for PgStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        use crate::schema::sessions::dsl::*;
        use diesel::{dsl::now, prelude::*};

        let conn = &mut self.db.get().map_err(adapt_backend_err)?;

        diesel::delete(sessions.filter(expiry_date.lt(now)))
            .execute(conn)
            .map_err(adapt_backend_err)?;
        Ok(())
    }
}

#[async_trait]
impl SessionStore for PgStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let conn = &mut self.db.get().map_err(adapt_backend_err)?;

        conn.transaction(|conn| {
            while self.id_exists(conn, &record.id)? {
                record.id = Id::default();
            }

            // remove nanoseconds data as PgSQL does not support precision over the millisecond
            let format = time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour]:[offset_minute]");
            let new_expiry = time::OffsetDateTime::parse(
                record.expiry_date
                    .format(format)
                    .map_err(adapt_serial_err)?
                    .as_str(),
                format)
                .map_err(adapt_serial_err)?;
            record.expiry_date = new_expiry;
            
            self.save_with_conn(conn, record)?;

            Ok(())
        }).map_err(adapt_diesel_err)
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let conn = &mut self.db.get().map_err(adapt_backend_err)?;

        self.save_with_conn(conn, record).map_err(adapt_diesel_err)
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        use crate::schema::sessions::dsl::*;
        use diesel::{prelude::*};

        let conn = &mut self.db.get().map_err(adapt_backend_err)?;

        let session = sessions
            .filter(id.eq(session_id.to_string()))
            .select(Session::as_select())
            .get_result(conn)
            .optional()
            .map_err(adapt_diesel_err)?
            .map(adapt_session_result);
        
        if let Some(session) = session {
            Ok(Some(session.map_err(adapt_backend_err)?))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        use crate::schema::sessions::dsl::*;
        use diesel::prelude::*;

        let conn = &mut self.db.get().map_err(adapt_backend_err)?;

        diesel::delete(sessions.filter(id.eq(session_id.to_string())))
            .execute(conn)
            .map_err(adapt_backend_err)?;
        Ok(())
    }
}

fn adapt_backend_err<T: std::error::Error>(error: T) ->  session_store::Error {
    session_store::Error::Backend(error.to_string())
}

fn adapt_diesel_err(error: diesel::result::Error) ->  session_store::Error {
    session_store::Error::Backend(error.to_string())
}

fn adapt_serial_err<T: std::error::Error + Send + Sync + 'static>(error: T) -> diesel::result::Error {
    diesel::result::Error::SerializationError(Box::new(error))
}

fn adapt_session_result(session: Session) -> Result<Record, rmp_serde::decode::Error> {
    Ok(Record {
        id: session.id.parse().unwrap(),
        data: rmp_serde::from_slice(&session.data)?,
        expiry_date: session.expiry_date,
    })
}

#[cfg(test)]
mod tests {
    use futures::FutureExt;
    use time::{Duration, OffsetDateTime};
    use crate::{Config};
    use crate::db::TestDb;
    use super::*;

    fn get_db_pool() -> TestDb {
        let config = Config::new().unwrap();
        TestDb::new(&config)
    }

    #[tokio::test]
    async fn test_create() {
        let db = get_db_pool();
        db.run_test(|pool| async move {
            let store = PgStore::new(pool);
            let mut record = Record {
                id: Default::default(),
                data: Default::default(),
                expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
            };
            assert!(store.create(&mut record).await.is_ok());
        }.boxed()).await;
    }

    #[tokio::test]
    async fn test_save() {
        let db = get_db_pool();
        db.run_test(|pool| async move {
            let store = PgStore::new(pool);
            let record = Record {
                id: Default::default(),
                data: Default::default(),
                expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
            };
            assert!(store.save(&record).await.is_ok());
        }.boxed()).await;
    }

    #[tokio::test]
    async fn test_load() {
        let db = get_db_pool();
        db.run_test(|pool| async move {
            let store = PgStore::new(pool);
            let mut record = Record {
                id: Default::default(),
                data: Default::default(),
                expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
            };
            store.create(&mut record).await.unwrap();
            let loaded_record = store.load(&record.id).await.unwrap();
            assert_eq!(Some(record), loaded_record);
        }.boxed()).await;
    }

    #[tokio::test]
    async fn test_delete() {
        let db = get_db_pool();
        db.run_test(|pool| async move {
            let store = PgStore::new(pool);
            let mut record = Record {
                id: Default::default(),
                data: Default::default(),
                expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
            };
            store.create(&mut record).await.unwrap();
            assert!(store.delete(&record.id).await.is_ok());
            assert_eq!(None, store.load(&record.id).await.unwrap());
        }.boxed()).await;
    }

    #[tokio::test]
    async fn test_create_id_collision() {
        let db = get_db_pool();
        db.run_test(|pool| async move {
            let store = PgStore::new(pool);
            let expiry_date = OffsetDateTime::now_utc() + Duration::minutes(30);
            let mut record1 = Record {
                id: Default::default(),
                data: Default::default(),
                expiry_date,
            };
            let mut record2 = Record {
                id: Default::default(),
                data: Default::default(),
                expiry_date,
            };
            store.create(&mut record1).await.unwrap();
            record2.id = record1.id; // Set the same ID for record2
            store.create(&mut record2).await.unwrap();
            assert_ne!(record1.id, record2.id); // IDs should be different
        }.boxed()).await;
    }
}
