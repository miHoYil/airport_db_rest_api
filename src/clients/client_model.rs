use crate::db;
use crate::error_handler::CustomError;
use crate::schema::Clients;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = Clients)]
pub struct ClientStruct {
    pub name: String,
	pub day_of_birth: chrono::NaiveDate,
	pub mail: String,
	pub passport: String,
}

impl ClientStruct {
    fn from(client: ClientStruct) -> ClientStruct {
        ClientStruct {
            name: client.name,
			day_of_birth: client.day_of_birth,
			mail: client.mail,
			passport: client.passport,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = Clients)]
pub struct ClientsStruct {
    pub id_client: i32,
    pub name: String,
	pub day_of_birth: chrono::NaiveDate,
	pub mail: String,
	pub passport: String,
}

impl ClientsStruct {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let clients = Clients::table.load::<ClientsStruct>(&mut conn)?;
        Ok(clients)
    }

    pub fn find(id_client: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let client = Clients::table.filter(Clients::id_client.eq(id_client)).first(&mut conn)?;
        Ok(client)
    }

    pub fn create(client: ClientStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let client = ClientStruct::from(client);
        let client = diesel::insert_into(Clients::table)
            .values(client)
			.on_conflict_do_nothing()
            .get_result(&mut conn)?;
        Ok(client)
    }

    pub fn update(id_client: i32, client: ClientStruct) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let client = diesel::update(Clients::table)
            .filter(Clients::id_client.eq(id_client))
            .set(client)
            .get_result(&mut conn)?;
        Ok(client)
    }

    pub fn delete(id_client: i32) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(Clients::table.filter(Clients::id_client.eq(id_client))).execute(&mut conn)?;
        Ok(res)
    }
}
