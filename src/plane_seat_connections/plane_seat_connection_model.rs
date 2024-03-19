use crate::db;
use crate::error_handler::CustomError;
use crate::schema::PlaneSeatConnections;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

//mod airplanes;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(belongs_to(AirPlanes))]
#[diesel(table_name = PlaneSeatConnections)]
pub struct PlaneSeatConnectionStruct {
    pub id_place_type: i32,
	pub id_plane: i32,
	pub seat_number: i32,
}

impl PlaneSeatConnectionStruct {
    fn from(plane_seat_connection: PlaneSeatConnectionStruct) -> PlaneSeatConnectionStruct {
        PlaneSeatConnectionStruct {
            id_place_type: plane_seat_connection.id_place_type,
			id_plane: plane_seat_connection.id_plane,
			seat_number: plane_seat_connection.seat_number,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = PlaneSeatConnections)]
pub struct PlaneSeatConnectionsStruct {
    pub id_plane_seat: i32,
    pub id_place_type: i32,
	pub id_plane: i32,
	pub seat_number: i32,
}

impl PlaneSeatConnectionsStruct {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let plane_seat_connections = PlaneSeatConnections::table.load::<PlaneSeatConnectionsStruct>(&mut conn)?;
        Ok(plane_seat_connections)
    }

    pub fn find(id_plane_seat: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let plane_seat_connection = PlaneSeatConnections::table.filter(PlaneSeatConnections::id_plane_seat.eq(id_plane_seat)).first(&mut conn)?;
        Ok(plane_seat_connection)
    }

    pub fn create(plane_seat_connection: PlaneSeatConnectionStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let plane_seat_connection = PlaneSeatConnectionStruct::from(plane_seat_connection);
        let plane_seat_connection = diesel::insert_into(PlaneSeatConnections::table)
            .values(plane_seat_connection)
			.on_conflict_do_nothing()
            .get_result(&mut conn)?;
        Ok(plane_seat_connection)
    }

    pub fn update(id_plane_seat: i32, plane_seat_connection: PlaneSeatConnectionStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let plane_seat_connection = diesel::update(PlaneSeatConnections::table)
            .filter(PlaneSeatConnections::id_plane_seat.eq(id_plane_seat))
            .set(plane_seat_connection)
            .get_result(&mut conn)?;
        Ok(plane_seat_connection)
    }

    pub fn delete(id_plane_seat: i32) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(PlaneSeatConnections::table.filter(PlaneSeatConnections::id_plane_seat.eq(id_plane_seat))).execute(&mut conn)?;
        Ok(res)
    }
	
	// pub fn find_all_free_flight(id_flight: i32) -> Result<Vec<SeatNumberStruct>, CustomError>{
		// let mut conn = db::connection()?;
		// let free_for_flight_seats = diesel::f
	// }
}
