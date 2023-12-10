use diesel::prelude::*;

use crate::{
    models::traffic_ticket::TrafficTicket,
    repository::database::{DbConnection, DbError},
};

pub fn get_traffic_tickets(conn: &mut DbConnection) -> Result<Vec<TrafficTicket>, DbError> {
    use crate::models::schema::traffic_tickets::dsl::*;

    let fetched = traffic_tickets.load::<TrafficTicket>(conn)?;
    Ok(fetched)
}

pub fn traffic_ticket_by(
    identifier: i32,
    conn: &mut DbConnection,
) -> Result<TrafficTicket, DbError> {
    use crate::models::schema::traffic_tickets::dsl::*;

    let fetched = traffic_tickets.find(identifier).first(conn)?;

    Ok(fetched)
}
