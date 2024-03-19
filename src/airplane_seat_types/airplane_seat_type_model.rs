use crate::db;
use crate::error_handler::CustomError;
use crate::schema::AirPlaneSeatTypes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = AirPlaneSeatTypes)]
pub struct AirPlaneSeatTypeStruct {
    pub type_name: String,
}

impl AirPlaneSeatTypeStruct {
    fn from(airplane_seat_type: AirPlaneSeatTypeStruct) -> AirPlaneSeatTypeStruct {
        AirPlaneSeatTypeStruct {
            type_name: airplane_seat_type.type_name,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = AirPlaneSeatTypes)]
pub struct AirPlaneSeatTypesStruct {
    pub id_placa_type: i32,
    pub type_name: String,
}

impl AirPlaneSeatTypesStruct {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let airplane_seat_types = AirPlaneSeatTypes::table.load::<AirPlaneSeatTypesStruct>(&mut conn)?;
        Ok(airplane_seat_types)
    }

    pub fn find(id_placa_type: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let airplane_seat_type = AirPlaneSeatTypes::table.filter(AirPlaneSeatTypes::id_placa_type.eq(id_placa_type)).first(&mut conn)?;
        Ok(airplane_seat_type)
    }

    pub fn create(airplane_seat_type: AirPlaneSeatTypeStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let airplane_seat_type = AirPlaneSeatTypeStruct::from(airplane_seat_type);
        let airplane_seat_type = diesel::insert_into(AirPlaneSeatTypes::table)
            .values(airplane_seat_type)
			.on_conflict_do_nothing()
            .get_result(&mut conn)?;
        Ok(airplane_seat_type)
    }

    pub fn update(id_placa_type: i32, airplane_seat_type: AirPlaneSeatTypeStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let airplane_seat_type = diesel::update(AirPlaneSeatTypes::table)
            .filter(AirPlaneSeatTypes::id_placa_type.eq(id_placa_type))
            .set(airplane_seat_type)
            .get_result(&mut conn)?;
        Ok(airplane_seat_type)
    }

    pub fn delete(id_placa_type: i32) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(AirPlaneSeatTypes::table.filter(AirPlaneSeatTypes::id_placa_type.eq(id_placa_type))).execute(&mut conn)?;
        Ok(res)
    }
}
