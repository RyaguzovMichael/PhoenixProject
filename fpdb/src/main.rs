mod db;
mod db_queries;
mod error;
mod models;

use crate::db::Database;
use chrono::Utc;
use fpcommon::{account::Account, category::Category, transaction::Transaction};
use std::path::Path;

fn main() -> Result<(), error::Error> {
    let db_path = Path::new("fp.db");
    let mut db = Database::new(db_path)?;

    let accounts = db.get_accounts()?;
    if accounts.is_empty() {
        let new_account = Account {
            name: String::from("My Bank"),
            description: None,
            currency: String::from("USD"),
        };
        println!("Adding account: {}", new_account.name);
        db.add_account(new_account)?;
    } else {
        println!("Found {} accounts.", accounts.len());
    }

    let categories = db.get_categories()?;
    if categories.is_empty() {
        let new_category = Category {
            name: String::from("Groceries"),
            description: None,
        };
        println!("Adding category: {}", new_category.name);
        db.add_category(new_category)?;
    } else {
        println!("Found {} categories.", categories.len());
    }

    let transactions = db.get_transactions()?;
    if transactions.is_empty() {
        let new_account = Account {
            name: String::from("My Bank"),
            description: None,
            currency: String::from("USD"),
        };
        let new_category = Category {
            name: String::from("Groceries"),
            description: None,
        };
        let new_transaction = Transaction::Income {
            amount: 10000,
            to: new_account,
            category: new_category,
            date: Utc::now(),
        };
        println!("Adding transaction");
        db.add_transaction(new_transaction)?;
    } else {
        println!("Found {} transactions.", transactions.len());
    }

    println!("Database is at {}", db_path.display());

    Ok(())
}
