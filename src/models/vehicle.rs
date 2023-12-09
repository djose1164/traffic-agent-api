use diesel::{Selectable, Queryable, Identifiable};
use serde::Serialize;
use crate::repository::database::DbBackend;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone, Serialize)]
#[diesel(table_name = super::schema::vehicles)]
#[diesel(check_for_backend(DbBackend))]
pub struct Vehicle {
    pub id: String,
    pub name: String,
    pub owner: String,
    motor_type: String,
    pub year: i32,
    pub color: String,
}
