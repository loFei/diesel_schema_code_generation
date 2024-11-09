//! DbAccountsService

use super::{ DbAccountsModel, NewDbAccountsModel, UpdateDbAccountsModel };
use crate::schema::{ self, accounts::dsl::* };
use diesel::prelude::*;

pub struct DbAccountsService;

impl DbAccountsService {
	pub fn list_accounts(conn: &mut SqliteConnection) -> Vec<DbAccountsModel> {
		match accounts.load::<DbAccountsModel>(conn) {
			Ok(list) => return list,
			Err(e) => {
				println!("Error loading accounts: {}", e);
		}
		}

			Vec::new()
	}

	pub fn add_accounts(conn: &mut SqliteConnection, new_model: &NewDbAccountsModel) -> DbAccountsModel {
		diesel::insert_into(schema::accounts::table)
			.values(new_model)
			.returning(DbAccountsModel::as_returning())
			.get_result(conn)
			.expect("Error saving new accounts")
	}

	pub fn find_accounts_by_id(conn: &mut SqliteConnection, model_id: i32) -> Option<DbAccountsModel> {
		accounts
		.find(model_id)
		.first(conn)
		.optional()
		.expect("Error loading accounts")
	}

	pub fn update_accounts_by_id(
		conn: &mut SqliteConnection,
		model_id: i32,
		update_model: &UpdateDbAccountsModel,
	) {
		diesel::update(accounts.find(model_id))
			.set(update_model)
			.execute(conn)
			.expect("Error updating accounts");
	}

	pub fn delete_accounts_by_id(conn: &mut SqliteConnection, model_id: i32) {
		diesel::delete(accounts.find(model_id))
			.execute(conn)
			.expect("Error deleting accounts");
	}
}

