use crate::db_queries::{ACCOUNT_SET, ACCOUNTS_GET, DB_SCEMA};
use crate::error::Error;
use crate::models::AccountDb;
use fpcommon::account::Account;
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
        self.conn.execute(DB_SCEMA, [])?;
        Ok(())
    }

    pub fn add_account(&mut self, account: Account) -> Result<(), Error> {
        let db_model = AccountDb::try_from(account)?;
        self.conn.execute(
            ACCOUNT_SET,
            params![
                db_model.primary_id,
                db_model.name,
                db_model.description,
                db_model.currency,
            ],
        )?;
        Ok(())
    }

    pub fn get_accounts(&self) -> Result<Vec<Account>, Error> {
        let mut stmt = self.conn.prepare(ACCOUNTS_GET)?;

        let accounts = stmt
            .query_map([], |row| {
                Ok(AccountDb {
                    primary_id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    currency: row.get(3)?,
                })
            })?
            .map(|el| AccountDb::try_into(el?))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(accounts)
    }
}
