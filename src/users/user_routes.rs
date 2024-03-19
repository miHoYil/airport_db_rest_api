
use crate::users::{UserDataStruct, UsersStruct};
use crate::error_handler::CustomError;

//put, delete, 
use actix_web::{get, web, HttpResponse};
//use serde_json::json;

#[get("/login")]
async fn check_user(user_data:  web::Json<UserDataStruct>) -> Result<HttpResponse, CustomError> {
    let user_role_str = UsersStruct::check_user(user_data.into_inner())?;
    Ok(HttpResponse::Ok().json(user_role_str))
}


pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(check_user);
}