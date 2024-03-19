
use crate::airports::{AirPortStruct, AirPortsStruct};
use crate::error_handler::CustomError;

//put, delete, 
use actix_web::{get, post, web, HttpResponse};
use serde_json::json;

#[get("/airports")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let airports = AirPortsStruct::find_all()?;
    Ok(HttpResponse::Ok().json(airports))
}

#[get("/airports/{id_airport}")]
async fn find(id_airport: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let airport = AirPortsStruct::find(id_airport.into_inner())?;
    Ok(HttpResponse::Ok().json(airport))
}

#[post("/airports/insert")]
async fn create(airport: web::Json<AirPortStruct>) -> Result<HttpResponse, CustomError> {
    let airport = AirPortsStruct::create(airport.into_inner())?;
    Ok(HttpResponse::Ok().json(airport))
}

#[post("/airports/update/{id_airport}")]
async fn update(
    id_airport: web::Path<i32>,
    airport: web::Json<AirPortStruct>,
) -> Result<HttpResponse, CustomError> {
    let airport = AirPortsStruct::update(id_airport.into_inner(), airport.into_inner())?;
    Ok(HttpResponse::Ok().json(airport))
}

#[get("/airports/delete/{id_airport}")]
async fn delete(id_airport: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_airport = AirPortsStruct::delete(id_airport.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_airport })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}