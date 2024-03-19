use crate::db;
use crate::error_handler::CustomError;
use crate::schema::FlightsSchedule;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::PlaneSeatConnections;
use crate::schema::Tickets;

//mod plane_seat_connections;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = FlightsSchedule)]
pub struct FlightsScheduleStruct {
    pub id_flight: i32,
    pub id_plane: i32,
	pub from_id_airport: i32,
	pub to_id_airport: i32,
	pub depature_date: chrono::NaiveDateTime,
	pub arrive_date: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = FlightsSchedule)]
pub struct FlightScheduleStruct {
    pub id_plane: i32,
	pub from_id_airport: i32,
	pub to_id_airport: i32,
	pub depature_date: chrono::NaiveDateTime,
	pub arrive_date: chrono::NaiveDateTime,
}

impl FlightScheduleStruct {
    fn from(flight_schedule: FlightScheduleStruct) -> FlightScheduleStruct {
        FlightScheduleStruct {
            id_plane: flight_schedule.id_plane,
			from_id_airport: flight_schedule.from_id_airport,
			to_id_airport: flight_schedule.to_id_airport,
			depature_date: flight_schedule.depature_date,
			arrive_date: flight_schedule.arrive_date,
        }
    }
}


impl FlightsScheduleStruct {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let flights_schedule = FlightsSchedule::table.load::<FlightsScheduleStruct>(&mut conn)?;
        Ok(flights_schedule)
    }

    pub fn find(id_flight: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let flight_schedule = FlightsSchedule::table.filter(FlightsSchedule::id_flight.eq(id_flight)).first(&mut conn)?;
        Ok(flight_schedule)
    }

    pub fn create(flight_schedule: FlightScheduleStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let flight_schedule = FlightScheduleStruct::from(flight_schedule);
        let flight_schedule = diesel::insert_into(FlightsSchedule::table)
            .values(flight_schedule)
			.on_conflict_do_nothing()
            .get_result(&mut conn)?;
        Ok(flight_schedule)
    }

    pub fn update(id_flight: i32, flight_schedule: FlightScheduleStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let flight_schedule = diesel::update(FlightsSchedule::table)
            .filter(FlightsSchedule::id_flight.eq(id_flight))
            .set(flight_schedule)
            .get_result(&mut conn)?;
        Ok(flight_schedule)
    }

    pub fn delete(id_flight: i32) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(FlightsSchedule::table.filter(FlightsSchedule::id_flight.eq(id_flight))).execute(&mut conn)?;
        Ok(res)
    }
		
	pub fn find_all_free_seat(id_flight: i32) -> Result<Vec<SeatNumberStruct>, CustomError>{
		let mut conn = db::connection()?;
		
		let id_plane_of_flight = 
			FlightsSchedule::table
				.filter(FlightsSchedule::id_flight.eq(id_flight))
				.select(FlightsSchedule::id_plane)
				.first::<i32>(&mut conn)?;
				
		let seats_in_flight = PlaneSeatConnections::table
			.filter(PlaneSeatConnections::id_plane.eq(id_plane_of_flight))
			.select(PlaneSeatConnections::seat_number)
			.load::<i32>(&mut conn)?;
		
		let seats_in_tickets = Tickets::table
			.filter(Tickets::id_flight.eq(id_flight))
			.select(Tickets::seat_number)
			.load::<i32>(&mut conn)?;
		
		let mut difference = vec![];
		for i in seats_in_flight {
			if !seats_in_tickets.contains(&i) {
				difference.push(i);
			}
		}
		
		let mut seats_struct: Vec<SeatNumberStruct> = vec![];
		for i in 0..difference.len()
		{
			seats_struct.push(SeatNumberStruct::from_i32(difference[i]));
		}
		
		Ok(seats_struct)	
	}
}

 //#[derive(Selectable, Queryable)]]
#[derive(Serialize)]
pub struct SeatNumberStruct
{
	pub seat_number: i32,
}
impl SeatNumberStruct
{
	fn from_i32(seat_number: i32) -> SeatNumberStruct
	{
		SeatNumberStruct{ 
		seat_number: seat_number,
		}
	}
}