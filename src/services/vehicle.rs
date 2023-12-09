use diesel::prelude::*;

use crate::{
    models::vehicle::Vehicle,
    repository::database::{DbConnection, DbError},
};

pub fn vehicle_by(id_: &str, conn: &mut DbConnection) -> Result<Vehicle, DbError> {
    use crate::models::schema::vehicles::dsl::*;

    let vehicle = vehicles
        .find(id_)
        .get_result(conn)?;


    // let owner = Owner::belonging_to(&vehicle)
    //     .get_result(conn)?;

    
    //let motor_type = 
        Ok(vehicle)
}
