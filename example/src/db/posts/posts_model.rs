//! DbPostsModel

use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DbPostsModel {
	pub id: i32, 
	pub title: String, 
	pub body: String, 
	pub published: bool, 
	pub poster_id: i32, 
	pub create_date: chrono::NaiveDateTime, 
	pub modify_date: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewDbPostsModel<'a> {
	pub title: &'a str,
	pub body: &'a str,
	pub published: bool,
	pub poster_id: i32,
	pub create_date: &'a chrono::NaiveDateTime,
	pub modify_date: Option<&'a chrono::NaiveDateTime>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::posts)]
pub struct UpdateDbPostsModel<'a> {
	pub title: Option<&'a str>,
	pub body: Option<&'a str>,
	pub published: Option<bool>,
	pub poster_id: Option<i32>,
	pub create_date: Option<&'a chrono::NaiveDateTime>,
	pub modify_date: Option<&'a chrono::NaiveDateTime>,
}


