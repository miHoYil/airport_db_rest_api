use crate::db;
use crate::error_handler::CustomError;
use crate::schema::Tickets;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::FlightsSchedule;
use crate::schema::AirPorts;
use crate::schema::AirPlanes;
use crate::schema::Clients;



#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = Tickets)]
pub struct TicketsStruct {
    pub id_ticket: i32,
    pub id_flight: i32,
	pub id_client: i32,
	pub seat_number: i32,
	pub baggage: bool,
	pub value_wo_b: f64,
	pub value_w_b: f64,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = Tickets)]
pub struct TicketStruct {
    pub id_flight: i32,
	pub id_client: i32,
	pub seat_number: i32,
	pub baggage: bool,
	pub value_wo_b: f64,
	pub value_w_b: f64,
}

impl TicketStruct {
    fn from(ticket: TicketStruct) -> TicketStruct {
        TicketStruct {
            id_flight: ticket.id_flight,
			id_client: ticket.id_client,
			seat_number: ticket.seat_number,
			baggage: ticket.baggage,
			value_wo_b: ticket.value_wo_b,
			value_w_b: ticket.value_w_b, 
        }
    }
}



impl TicketsStruct {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let tickets = Tickets::table.load::<TicketsStruct>(&mut conn)?;
        Ok(tickets)
    }

    pub fn find(id_ticket: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let ticket = Tickets::table.filter(Tickets::id_ticket.eq(id_ticket)).first(&mut conn)?;
        Ok(ticket)
    }

    pub fn create(ticket: TicketStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let ticket = TicketStruct::from(ticket);
        let ticket = diesel::insert_into(Tickets::table)
            .values(ticket)
			.on_conflict_do_nothing()
            .get_result(&mut conn)?;
        Ok(ticket)
    }

    pub fn update(id_ticket: i32, ticket: TicketStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let ticket = diesel::update(Tickets::table)
            .filter(Tickets::id_ticket.eq(id_ticket))
            .set(ticket)
            .get_result(&mut conn)?;
        Ok(ticket)
    }

    pub fn delete(id_ticket: i32) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(Tickets::table.filter(Tickets::id_ticket.eq(id_ticket))).execute(&mut conn)?;
        Ok(res)
    }
	
	pub fn all_info_about_ticket(id_ticket: i32) -> Result<AllDataAboutTicketStruct, CustomError>{
		let mut conn = db::connection()?;
		
		let from_airport_str = Tickets::table
							   .filter(Tickets::id_ticket.eq(id_ticket))
							   .inner_join(
									FlightsSchedule::table.inner_join(
										AirPorts::table.on(FlightsSchedule::to_id_airport.eq(AirPorts::id_airport))
									)
								)
								.select(AirPorts::airport_name)
								.load::<String>(&mut conn)?;
								
		if from_airport_str.len() < 1 {
			return Err(CustomError::new(404, "Ticket Doesn't Exist".to_string()));	
		}
		
		let data = Tickets::table
				   .filter(Tickets::id_ticket.eq(id_ticket))
				   .inner_join(
						FlightsSchedule::table.inner_join(
							AirPorts::table.on(FlightsSchedule::from_id_airport.eq(AirPorts::id_airport))
						)
						.inner_join(AirPlanes::table)
					)				   
				   .inner_join(Clients::table)
				   .select((Tickets::id_ticket, Tickets::id_flight, AirPlanes::plane_name, Clients::name, 
							Tickets::seat_number, AirPorts::airport_name, FlightsSchedule::depature_date, FlightsSchedule::arrive_date))
				   .load::<(i32, i32, String,  String, i32, String, chrono::NaiveDateTime, chrono::NaiveDateTime)>(&mut conn)?;
		
		let mut data_struct = AllDataAboutTicketStruct::all_data_except_from_airport(data[0].clone());
		
		
								
		data_struct.set_from_airport(from_airport_str[0].clone());
		
		Ok(data_struct)
	}
}

						// .union(
							// FlightsSchedule::table.inner_join(
								// AirPorts::table.on(FlightsSchedule::to_id_airport.eq(AirPorts::id_airport))
							// )
						// )
						
#[derive(Serialize)]
pub struct AllDataAboutTicketStruct{
	pub id_ticket: i32,
    pub id_flight: i32,
	pub plane_name: String,
	pub client_name: String,
	pub seat_number: i32,
	pub to_airport_name: String,
	pub from_airport_name: String,
	pub depature_date: chrono::NaiveDateTime,
	pub arrive_date: chrono::NaiveDateTime,
}
impl AllDataAboutTicketStruct{
	pub fn all_data_except_from_airport((id_ticket, id_flight, plane_name, client_name, seat_number, to_airport_name, depature_date, arrive_date)
		:(i32, i32, String, String, i32, String, chrono::NaiveDateTime, chrono::NaiveDateTime)) -> AllDataAboutTicketStruct{
			AllDataAboutTicketStruct
			{
				id_ticket: id_ticket,
				id_flight: id_flight,
				plane_name: plane_name,
				client_name: client_name,
				seat_number: seat_number,
				to_airport_name: to_airport_name,
				from_airport_name: "".to_string(),
				depature_date: depature_date,
				arrive_date: arrive_date,
			}
	}
	
	pub fn set_from_airport(&mut self, from_airport_name: String)
	{
		self.from_airport_name = from_airport_name;
	}
}
