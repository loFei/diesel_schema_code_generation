//! DbPostsService

use super::{ DbPostsModel, NewDbPostsModel, UpdateDbPostsModel };
use crate::schema::{ self, posts::dsl::* };
use diesel::prelude::*;

pub struct DbPostsService;

impl DbPostsService {
	pub fn list_posts(conn: &mut SqliteConnection) -> Vec<DbPostsModel> {
		match posts.load::<DbPostsModel>(conn) {
			Ok(list) => return list,
			Err(e) => {
				println!("Error loading posts: {}", e);
			}
		}

		Vec::new()
	}

	pub fn add_posts(conn: &mut SqliteConnection, new_model: &NewDbPostsModel) -> DbPostsModel {
		diesel::insert_into(schema::posts::table)
			.values(new_model)
			.returning(DbPostsModel::as_returning())
			.get_result(conn)
			.expect("Error saving new posts")
	}

	pub fn find_posts_by_id(conn: &mut SqliteConnection, model_id: i32) -> Option<DbPostsModel> {
		posts.find(model_id)
			.first(conn)
			.optional()
			.expect("Error loading posts")
	}

	pub fn update_posts_by_id(
		conn: &mut SqliteConnection,
		model_id: i32,
		update_model: &UpdateDbPostsModel,
	) {
		diesel::update(posts.find(model_id))
			.set(update_model)
			.execute(conn)
			.expect("Error updating posts");
	}

	pub fn delete_posts_by_id(conn: &mut SqliteConnection, model_id: i32) {
		diesel::delete(posts.find(model_id))
			.execute(conn)
			.expect("Error deleting posts");
	}
}

