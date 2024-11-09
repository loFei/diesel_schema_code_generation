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


