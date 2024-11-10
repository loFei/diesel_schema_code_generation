//! DbAccountsModel

use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DbAccountsModel {
	pub id: i32, 
	pub user_name: String, 
	pub register_date: chrono::NaiveDateTime, 
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::accounts)]
pub struct NewDbAccountsModel<'a> {
	pub user_name: &'a str,
	pub register_date: &'a chrono::NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::accounts)]
pub struct UpdateDbAccountsModel<'a> {
	pub user_name: Option<&'a str>,
	pub register_date: Option<&'a chrono::NaiveDateTime>,
}

impl<'a> UpdateDbAccountsModel<'a> {
	pub fn create() -> Self{
		Self {
			user_name: None,
			register_date: None,
		}
	}
	pub fn set_user_name(&mut self, value: &'a str) {
		self.user_name = Some(value);
	}

	pub fn set_register_date(&mut self, value: &'a chrono::NaiveDateTime) {
		self.register_date = Some(value);
	}

}


