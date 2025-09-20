use crate::db_queries::{
    ACCOUNT_SET, ACCOUNTS_GET, CATEGORIES_GET, CATEGORY_SET, DB_SCEMA, TRANSACTION_SET,
    TRANSACTIONS_GET,
};
use crate::error::Error;
use crate::models::TransactionDb;
use fpcommon::account::Account;
use fpcommon::category::Category;
use fpcommon::transaction::Transaction;
use rusqlite::{Connection, params};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        // need create_tables if not exist
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<(), Error> {
        self.conn.execute_batch(DB_SCEMA)?;
        Ok(())
    }

    pub fn add_account(&mut self, account: Account) -> Result<(), Error> {
        self.conn.execute(
            ACCOUNT_SET,
            params![account.name, account.description, account.currency,],
        )?;
        Ok(())
    }

    pub fn get_accounts(&self) -> Result<Vec<Account>, Error> {
        let mut stmt = self.conn.prepare(ACCOUNTS_GET)?;

        let accounts = stmt
            .query_map([], |row| {
                Ok(Account {
                    name: row.get(0)?,
                    description: row.get(1)?,
                    currency: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(accounts)
    }

    pub fn add_category(&mut self, category: Category) -> Result<(), Error> {
        self.conn
            .execute(CATEGORY_SET, params![category.name, category.description,])?;
        Ok(())
    }

    pub fn get_categories(&self) -> Result<Vec<Category>, Error> {
        let mut stmt = self.conn.prepare(CATEGORIES_GET)?;

        let categories = stmt
            .query_map([], |row| {
                Ok(Category {
                    name: row.get(0)?,
                    description: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), Error> {
        let db_model = TransactionDb::from(transaction);
        self.conn.execute(
            TRANSACTION_SET,
            params![
                db_model.amount,
                db_model.date,
                db_model.category_name,
                db_model.currency_rate,
                db_model.from_name,
                db_model.to_name,
            ],
        )?;
        Ok(())
    }

    pub fn get_transactions(&self) -> Result<Vec<Transaction>, Error> {
        let mut stmt = self.conn.prepare(TRANSACTIONS_GET)?;

        let transactions = stmt
            .query_map([], |row| {
                Ok(TransactionDb {
                    amount: row.get(0)?,
                    date: row.get(1)?,
                    category_name: row.get(2)?,
                    category_description: row.get(3)?,
                    currency_rate: row.get(4)?,
                    from_name: row.get(5)?,
                    from_description: row.get(6)?,
                    from_currency: row.get(7)?,
                    to_name: row.get(8)?,
                    to_description: row.get(9)?,
                    to_currency: row.get(10)?,
                })
            })?
            .map(|el| TransactionDb::try_into(el?))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(transactions)
    }
}
