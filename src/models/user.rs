use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Selectable};
use serde::{Deserialize, Serialize};

use crate::repository::database::DbBackend;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(DbBackend))]
pub struct User {
    id: i32,
    name: String,
    last_name: Option<String>,
    pub email: Option<String>,
    pub password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPayload {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCredentials {
    pub email: String,
    pub password: String,
}
