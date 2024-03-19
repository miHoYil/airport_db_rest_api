
use crate::airplane_seat_types::{AirPlaneSeatTypeStruct, AirPlaneSeatTypesStruct};
use crate::error_handler::CustomError;

//put, delete, 
use actix_web::{get, post, web, HttpResponse};
use serde_json::json;

#[get("/airplane_seat_types")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let airplane_seat_types = AirPlaneSeatTypesStruct::find_all()?;
    Ok(HttpResponse::Ok().json(airplane_seat_types))
}

#[get("/airplane_seat_types/{id_placa_type}")]
async fn find(id_placa_type: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let airplane_seat_type = AirPlaneSeatTypesStruct::find(id_placa_type.into_inner())?;
    Ok(HttpResponse::Ok().json(airplane_seat_type))
}

#[post("/airplane_seat_types/insert")]
async fn create(airplane_seat_type: web::Json<AirPlaneSeatTypeStruct>) -> Result<HttpResponse, CustomError> {
    let airplane_seat_type = AirPlaneSeatTypesStruct::create(airplane_seat_type.into_inner())?;
    Ok(HttpResponse::Ok().json(airplane_seat_type))
}

#[post("/airplane_seat_types/update/{id_placa_type}")]
async fn update(
    id_placa_type: web::Path<i32>,
    airplane_seat_type: web::Json<AirPlaneSeatTypeStruct>,
) -> Result<HttpResponse, CustomError> {
    let airplane_seat_type = AirPlaneSeatTypesStruct::update(id_placa_type.into_inner(), airplane_seat_type.into_inner())?;
    Ok(HttpResponse::Ok().json(airplane_seat_type))
}

#[get("/airplane_seat_types/delete/{id_placa_type}")]
async fn delete(id_placa_type: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_airplane_seat_type = AirPlaneSeatTypesStruct::delete(id_placa_type.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_airplane_seat_type })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}