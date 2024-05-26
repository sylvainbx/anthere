use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, Insertable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: String,
    pub data: Vec<u8>,
    pub expiry_date: time::OffsetDateTime,
}