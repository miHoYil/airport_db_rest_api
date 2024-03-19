
use crate::tickets::{TicketStruct, TicketsStruct};
use crate::error_handler::CustomError;

//put, delete, 
use actix_web::{get, post, web, HttpResponse};
use serde_json::json;

#[get("/tickets")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let tickets = TicketsStruct::find_all()?;
    Ok(HttpResponse::Ok().json(tickets))
}

#[get("/tickets/{id_ticket}")]
async fn find(id_ticket: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let ticket = TicketsStruct::find(id_ticket.into_inner())?;
    Ok(HttpResponse::Ok().json(ticket))
}

#[post("/tickets/insert")]
async fn create(ticket: web::Json<TicketStruct>) -> Result<HttpResponse, CustomError> {
    let ticket = TicketsStruct::create(ticket.into_inner())?;
    Ok(HttpResponse::Ok().json(ticket))
}

#[post("/tickets/update/{id_ticket}")]
async fn update(
    id_ticket: web::Path<i32>,
    ticket: web::Json<TicketStruct>,
) -> Result<HttpResponse, CustomError> {
    let ticket = TicketsStruct::update(id_ticket.into_inner(), ticket.into_inner())?;
    Ok(HttpResponse::Ok().json(ticket))
}

#[get("/tickets/delete/{id_ticket}")]
async fn delete(id_ticket: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_client = TicketsStruct::delete(id_ticket.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_client })))
}

#[get("/tickets/allData/{id_ticket}")]
async fn all_info_about_ticket(id_ticket: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let data = TicketsStruct::all_info_about_ticket(id_ticket.into_inner())?;
    Ok(HttpResponse::Ok().json(data))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
	comfig.service(all_info_about_ticket);
}