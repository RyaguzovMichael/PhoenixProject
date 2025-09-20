mod db;
mod db_queries;
mod error;
mod models;

use crate::db::Database;
use fpcommon::account::Account;
use std::path::Path;

fn main() -> Result<(), error::Error> {
    let db_path = Path::new("fp.db");
    let mut db = Database::new(db_path)?;

    let accounts = db.get_accounts()?;
    if accounts.is_empty() {
        let new_account = Account::new(String::from("My Bank"), None, String::from("USD"));
        println!("Adding account: {}", new_account.name);
        db.add_account(new_account)?;
    } else {
        println!("Found {} accounts.", accounts.len());
    }

    println!("Database is at {}", db_path.display());

    Ok(())
}
