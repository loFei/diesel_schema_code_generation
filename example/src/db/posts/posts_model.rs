//! DbPostsModel

use chrono::Local;
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
	pub create_date: chrono::NaiveDateTime,
	pub modify_date: Option<chrono::NaiveDateTime>,
}

impl<'a> NewDbPostsModel<'a> {
	pub fn create() -> Self {
		Self {
			title: "",
			body: "",
			published: false,
			poster_id: 0,
			create_date: Local::now().naive_local(),
			modify_date: None,
		}
	}

	pub fn set_title(&mut self, value: &'a str) {
		self.title = value;
	}

	pub fn set_body(&mut self, value: &'a str) {
		self.body = value;
	}

	pub fn set_published(&mut self, value: bool) {
		self.published = value;
	}

	pub fn set_poster_id(&mut self, value: i32) {
		self.poster_id = value;
	}

	pub fn set_create_date(&mut self, value: chrono::NaiveDateTime) {
		self.create_date = value;
	}

	pub fn set_modify_date(&mut self, value: chrono::NaiveDateTime) {
		self.modify_date = Some(value);
	}

}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::posts)]
pub struct UpdateDbPostsModel<'a> {
	pub title: Option<&'a str>,
	pub body: Option<&'a str>,
	pub published: Option<bool>,
	pub poster_id: Option<i32>,
	pub create_date: Option<chrono::NaiveDateTime>,
	pub modify_date: Option<chrono::NaiveDateTime>,
}

impl<'a> UpdateDbPostsModel<'a> {
	pub fn create() -> Self {
		Self {
			title: None,
			body: None,
			published: None,
			poster_id: None,
			create_date: None,
			modify_date: None,
		}
	}

	pub fn set_title(&mut self, value: &'a str) {
		self.title = Some(value);
	}

	pub fn set_body(&mut self, value: &'a str) {
		self.body = Some(value);
	}

	pub fn set_published(&mut self, value: bool) {
		self.published = Some(value);
	}

	pub fn set_poster_id(&mut self, value: i32) {
		self.poster_id = Some(value);
	}

	pub fn set_create_date(&mut self, value: chrono::NaiveDateTime) {
		self.create_date = Some(value);
	}

	pub fn set_modify_date(&mut self, value: chrono::NaiveDateTime) {
		self.modify_date = Some(value);
	}

}


