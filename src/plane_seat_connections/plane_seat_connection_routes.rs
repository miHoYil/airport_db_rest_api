
use crate::plane_seat_connections::{PlaneSeatConnectionStruct, PlaneSeatConnectionsStruct};
use crate::error_handler::CustomError;

//put, delete, 
use actix_web::{get, post, web, HttpResponse};
use serde_json::json;

#[get("/plane_seat_connections")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let plane_seat_connections = PlaneSeatConnectionsStruct::find_all()?;
    Ok(HttpResponse::Ok().json(plane_seat_connections))
}

#[get("/plane_seat_connections/{id_plane_seat}")]
async fn find(id_plane_seat: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let plane_seat_connection = PlaneSeatConnectionsStruct::find(id_plane_seat.into_inner())?;
    Ok(HttpResponse::Ok().json(plane_seat_connection))
}

#[post("/plane_seat_connections/insert")]
async fn create(plane_seat_connection: web::Json<PlaneSeatConnectionStruct>) -> Result<HttpResponse, CustomError> {
    let plane_seat_connection = PlaneSeatConnectionsStruct::create(plane_seat_connection.into_inner())?;
    Ok(HttpResponse::Ok().json(plane_seat_connection))
}

#[post("/plane_seat_connections/update/{id_plane_seat}")]
async fn update(
    id_plane_seat: web::Path<i32>,
    plane_seat_connection: web::Json<PlaneSeatConnectionStruct>,
) -> Result<HttpResponse, CustomError> {
    let plane_seat_connection = PlaneSeatConnectionsStruct::update(id_plane_seat.into_inner(), plane_seat_connection.into_inner())?;
    Ok(HttpResponse::Ok().json(plane_seat_connection))
}

#[get("/plane_seat_connections/delete/{id_plane_seat}")]
async fn delete(id_plane_seat: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_client = PlaneSeatConnectionsStruct::delete(id_plane_seat.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_client })))
}

// #[get("/plane_seat_connections/flight/{id_flight}")]
// async fn find_all_free_flight(id_flight: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    // let free_seat_numbers = PlaneSeatConnectionsStruct::find_all_free_flight()?;
    // Ok(HttpResponse::Ok().json(free_seat_numbers))
// }


pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
	//comfig.service(find_all_free_flight);
}