
use crate::clients::{ClientStruct, ClientsStruct};
use crate::error_handler::CustomError;

//put, delete, 
use actix_web::{get, post, web, HttpResponse};
use serde_json::json;

#[get("/clients")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let clients = ClientsStruct::find_all()?;
    Ok(HttpResponse::Ok().json(clients))
}

#[get("/clients/{id_client}")]
async fn find(id_client: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let client = ClientsStruct::find(id_client.into_inner())?;
    Ok(HttpResponse::Ok().json(client))
}

#[post("/clients/insert")]
async fn create(client: web::Json<ClientStruct>) -> Result<HttpResponse, CustomError> {
    let client = ClientsStruct::create(client.into_inner())?;
    Ok(HttpResponse::Ok().json(client))
}

#[post("/clients/update/{id_client}")]
async fn update(
    id_client: web::Path<i32>,
    client: web::Json<ClientStruct>,
) -> Result<HttpResponse, CustomError> {
    let client = ClientsStruct::update(id_client.into_inner(), client.into_inner())?;
    Ok(HttpResponse::Ok().json(client))
}

#[get("/clients/delete/{id_client}")]
async fn delete(id_client: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_client = ClientsStruct::delete(id_client.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_client })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}