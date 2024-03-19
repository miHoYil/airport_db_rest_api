use crate::db;
use crate::error_handler::CustomError;

use crate::schema::Users;
use crate::schema::UserTypes;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = UserTypes)]
pub struct UserTypesStruct {
	pub id_user_type: i32,
	pub type_name: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = UserTypes)]
pub struct UserTypeStruct {
	pub type_name: String,

}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = Users)]
pub struct UsersStruct {
	pub id_user: i32,
	pub name: String,
	pub password: String,
	pub id_user_type: i32,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = Users)]
pub struct UserDataStruct {
	pub name: String,
	pub password: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = Users)]
pub struct UserStruct {
	pub name: String,
	pub password: String,
	pub id_user_type: i32,
}

impl UsersStruct {
    pub fn check_user(user_data: UserDataStruct) -> Result<UserTypeStruct, CustomError> {
        let mut conn = db::connection()?;
		
        let user_role = Users::table
						.filter(Users::name.eq(user_data.name))
						.filter(Users::password.eq(user_data.password))
						.select(Users::id_user_type)
						.first::<i32>(&mut conn)?;
		
			// if user_role.len() != 1 {
				// return Err(CustomError::new(500, "Error: Invalid credentials".to_string()));	
			// }
		
		let user_role_str = UserTypes::table
							.filter(UserTypes::id_user_type.eq(user_role))
							.select(UserTypes::type_name)
							.first::<String>(&mut conn)?;
		
        Ok(UserTypeStruct{
			type_name: user_role_str,
		})
    }
}
