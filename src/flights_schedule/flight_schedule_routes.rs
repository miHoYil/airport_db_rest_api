
use crate::flights_schedule::{FlightScheduleStruct, FlightsScheduleStruct};
use crate::error_handler::CustomError;

//put, delete, 
use actix_web::{get, post, web, HttpResponse};
use serde_json::json;

#[get("/flights_schedule")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let flights_schedule = FlightsScheduleStruct::find_all()?;
    Ok(HttpResponse::Ok().json(flights_schedule))
}

#[get("/flights_schedule/{id_flight}")]
async fn find(id_flight: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let flight_schedule = FlightsScheduleStruct::find(id_flight.into_inner())?;
    Ok(HttpResponse::Ok().json(flight_schedule))
}

#[post("/flights_schedule/insert")]
async fn create(flight_schedule: web::Json<FlightScheduleStruct>) -> Result<HttpResponse, CustomError> {
    let flight_schedule = FlightsScheduleStruct::create(flight_schedule.into_inner())?;
    Ok(HttpResponse::Ok().json(flight_schedule))
}

#[post("/flights_schedule/update/{id_flight}")]
async fn update(
    id_flight: web::Path<i32>,
    flight_schedule: web::Json<FlightScheduleStruct>,
) -> Result<HttpResponse, CustomError> {
    let flight_schedule = FlightsScheduleStruct::update(id_flight.into_inner(), flight_schedule.into_inner())?;
    Ok(HttpResponse::Ok().json(flight_schedule))
}

#[get("/flights_schedule/delete/{id_flight}")]
async fn delete(id_flight: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_client = FlightsScheduleStruct::delete(id_flight.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_client })))
}

#[get("/flights_schedule/seats/{id_flight}")]
async fn find_all_free_seat(id_flight: web::Path<i32>) -> Result<HttpResponse, CustomError>
{
	let seats = FlightsScheduleStruct::find_all_free_seat(id_flight.into_inner())?;
	Ok(HttpResponse::Ok().json(seats))
}


pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
	comfig.service(find_all_free_seat);
}