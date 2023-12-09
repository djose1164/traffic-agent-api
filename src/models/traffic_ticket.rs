use diesel::{Queryable, Selectable, Insertable};
use serde::{Serialize, Deserialize};

use crate::repository::database::DbBackend;

#[derive(Serialize, Deserialize, Debug, Queryable, Selectable)]
#[diesel(table_name = super::schema::traffic_tickets)]
#[diesel(check_for_backend(DbBackend))]
pub struct TrafficTicket {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::traffic_tickets)]
pub struct NewTrafficTicket<'a> {
    pub name: &'a str,
    pub description: &'a str
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrafficTicketPayload {
    pub name: String,
    pub description: String
}