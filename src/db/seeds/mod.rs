use diesel::{insert_into, PgConnection, prelude::*};

pub mod users;

pub fn seeds(conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
   conn.transaction(|conn| {
      insert_into(crate::schema::users::table).values(&users::users()).execute(conn)?;
      
      Ok(())
   })
   
}