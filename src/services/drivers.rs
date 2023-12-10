use diesel::prelude::*;

use crate::{models::driver::Driver, repository::database::DbError};

use crate::repository::database::DbConnection;

pub fn read(identifier: i32, conn: &mut DbConnection) -> Result<Driver, DbError> {
    use crate::models::schema::drivers::dsl::*;

    let fetched = drivers.find(identifier).first(conn)?;

    Ok(fetched)
}
