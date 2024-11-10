mod db;

use chrono::Local;
use db::*;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let conn = &mut establish_connection();

    let reg_date = Local::now().naive_local();

    let mut new_account = NewDbAccountsModel::create();
    new_account.user_name = "Alice";
    new_account.register_date = reg_date;
    let acc_data = DbAccountsService::add_accounts(conn, &new_account);
    println!(
        "==> add_account account result:\nid: {}, user_name: {}, register_date: {}",
        acc_data.id, acc_data.user_name, acc_data.register_date
    );

    let account_list = DbAccountsService::list_accounts(conn);
    if account_list.is_empty() {
        println!("No accounts data");
    } else {
        for acc in account_list.iter() {
            println!(
                "Account id: {}, user_name: {}, register_date: {}",
                acc.id, acc.user_name, acc.register_date
            );
        }
    }

    let mut update_account = UpdateDbAccountsModel::create();
    update_account.user_name = Some("Angle");
    DbAccountsService::update_accounts_by_id(conn, acc_data.id, &update_account);
    if let Some(acc) = DbAccountsService::find_accounts_by_id(conn, acc_data.id) {
        println!(
            "==> update account result:\n id: {}, user_name: {}, register_date: {}",
            acc.id, acc.user_name, acc.register_date
        );
    }

    println!("==> Delete account id: {}", acc_data.id);
    DbAccountsService::delete_accounts_by_id(conn, acc_data.id);

    match DbAccountsService::find_accounts_by_id(conn, acc_data.id) {
        Some(acc) => println!(
            "==> get account result:\nid: {}, user_name: {}, register_date: {}",
            acc.id, acc.user_name, acc.register_date
        ),
        None => println!("==> Not found account by id: {}", acc_data.id),
    }
}
