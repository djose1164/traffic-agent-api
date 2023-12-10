use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;

use crate::repository::database::DbBackend;

#[derive(Serialize, Selectable, Identifiable, Queryable)]
#[diesel(check_for_backend(DbBackend))]
#[diesel(table_name = super::schema::drivers)]
pub struct Driver {
    pub id: i32,
    pub name: String,
    pub birth_date: NaiveDate,
    pub address: String,
    pub phone_number: String,
    pub picture: String
}