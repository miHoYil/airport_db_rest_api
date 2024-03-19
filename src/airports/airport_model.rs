use crate::db;
use crate::error_handler::CustomError;
use crate::schema::AirPorts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = AirPorts)]
pub struct AirPortStruct {
    pub airport_name: String,
	pub airport_address: String,
}

impl AirPortStruct {
    fn from(airport: AirPortStruct) -> AirPortStruct {
        AirPortStruct {
            airport_name: airport.airport_name,
			airport_address: airport.airport_address,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = AirPorts)]
pub struct AirPortsStruct {
    pub id_airport: i32,
    pub airport_name: String,
	pub airport_address: String,
}

impl AirPortsStruct {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let airports = AirPorts::table.load::<AirPortsStruct>(&mut conn)?;
        Ok(airports)
    }

    pub fn find(id_airport: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let airport = AirPorts::table.filter(AirPorts::id_airport.eq(id_airport)).first(&mut conn)?;
        Ok(airport)
    }

    pub fn create(airport: AirPortStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let airport = AirPortStruct::from(airport);
        let airport = diesel::insert_into(AirPorts::table)
            .values(airport)
			.on_conflict_do_nothing()
            .get_result(&mut conn)?;
        Ok(airport)
    }

    pub fn update(id_airport: i32, airport: AirPortStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let airport = diesel::update(AirPorts::table)
            .filter(AirPorts::id_airport.eq(id_airport))
            .set(airport)
            .get_result(&mut conn)?;
        Ok(airport)
    }

    pub fn delete(id_airport: i32) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(AirPorts::table.filter(AirPorts::id_airport.eq(id_airport))).execute(&mut conn)?;
        Ok(res)
    }
}
