use crate::models::user::{User, UserPayload, NewUser};
use crate::repository::database::{DbError, DbConnection};
use diesel::prelude::*;

pub fn authenticate_user(user: UserPayload, conn: &mut DbConnection) -> Result<User, DbError> {
    use crate::models::schema::users::dsl::*;

    let fetched = users
        .filter(email.eq(user.email).and(password.eq(user.password)))
        .first(conn)?;

    Ok(fetched)
}

pub fn create_user(user: UserPayload, conn: &mut DbConnection) -> Result<User, DbError> {
    use crate::models::schema::users::dsl::*;

    let new_user = NewUser {
        name: &user.name,
        last_name: &user.last_name,
        email: &user.email,
        password: &user.password
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error inserting user");

    let fetched = users
        .order(id.desc())
        .limit(1)
        .first(conn)?;

    Ok(fetched)
}
