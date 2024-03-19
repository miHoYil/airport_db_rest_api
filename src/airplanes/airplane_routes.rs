
use crate::airplanes::{AirPlaneStruct, AirPlanesStruct};
use crate::error_handler::CustomError;

//put, delete, 
use actix_web::{get, post, web, HttpResponse};
use serde_json::json;

#[get("/airplanes")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let airplanes = AirPlanesStruct::find_all()?;
    Ok(HttpResponse::Ok().json(airplanes))
}

#[get("/airplanes/{id_plane}")]
async fn find(id_plane: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let airplane = AirPlanesStruct::find(id_plane.into_inner())?;
    Ok(HttpResponse::Ok().json(airplane))
}

#[post("/airplanes/insert")]
async fn create(airplane: web::Json<AirPlaneStruct>) -> Result<HttpResponse, CustomError> {
    let airplane = AirPlanesStruct::create(airplane.into_inner())?;
    Ok(HttpResponse::Ok().json(airplane))
}

#[post("/airplanes/update/{id_plane}")]
async fn update(
    id_plane: web::Path<i32>,
    airplane: web::Json<AirPlaneStruct>,
) -> Result<HttpResponse, CustomError> {
    let airplane = AirPlanesStruct::update(id_plane.into_inner(), airplane.into_inner())?;
    Ok(HttpResponse::Ok().json(airplane))
}

#[get("/airplanes/delete/{id_plane}")]
async fn delete(id_plane: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_airplane = AirPlanesStruct::delete(id_plane.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_airplane })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}