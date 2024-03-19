use crate::db;
use crate::error_handler::CustomError;
use crate::schema::AirPlanes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = AirPlanes)]
pub struct AirPlaneStruct {
    pub plane_name: String,
}

impl AirPlaneStruct {
    fn from(airplane: AirPlaneStruct) -> AirPlaneStruct {
        AirPlaneStruct {
            plane_name: airplane.plane_name,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = AirPlanes)]
pub struct AirPlanesStruct {
    pub id_plane: i32,
    pub plane_name: String,
}

impl AirPlanesStruct {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let airplanes = AirPlanes::table.load::<AirPlanesStruct>(&mut conn)?;
        Ok(airplanes)
    }

    pub fn find(id_plane: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let airplane = AirPlanes::table.filter(AirPlanes::id_plane.eq(id_plane)).first(&mut conn)?;
        Ok(airplane)
    }

    pub fn create(airplane: AirPlaneStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let airplane = AirPlaneStruct::from(airplane);
        let airplane = diesel::insert_into(AirPlanes::table)
            .values(airplane)
			.on_conflict_do_nothing()
            .get_result(&mut conn)?;
        Ok(airplane)
    }

    pub fn update(id_plane: i32, airplane: AirPlaneStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let airplane = diesel::update(AirPlanes::table)
            .filter(AirPlanes::id_plane.eq(id_plane))
            .set(airplane)
            .get_result(&mut conn)?;
        Ok(airplane)
    }

    pub fn delete(id_plane: i32) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(AirPlanes::table.filter(AirPlanes::id_plane.eq(id_plane))).execute(&mut conn)?;
        Ok(res)
    }
}
