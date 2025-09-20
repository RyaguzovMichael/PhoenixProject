use crate::error::Error;
use chrono::{DateTime, Utc};
use fpcommon::{
    account::Account, category::Category, currency::CurrencyRate, transaction::Transaction,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

trait DataBaseItem {
    fn get_id(&self) -> &str;
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct AccountDb {
    pub primary_id: String,
    pub name: String,
    pub description: Option<String>,
    pub currency: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct TransactionDb {
    pub primary_id: String,
    pub amount: i64,
    pub category_id: Option<String>,
    pub currency_rate: Option<f64>,
    pub from_id: Option<String>,
    pub to_id: Option<String>,
    pub date: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct CategoryDb {
    pub primary_id: String,
    pub name: String,
    pub description: Option<String>,
}

impl DataBaseItem for AccountDb {
    fn get_id(&self) -> &str {
        &self.primary_id
    }
}

impl DataBaseItem for CategoryDb {
    fn get_id(&self) -> &str {
        &self.primary_id
    }
}

impl DataBaseItem for TransactionDb {
    fn get_id(&self) -> &str {
        &self.primary_id
    }
}

impl From<Account> for AccountDb {
    fn from(account: Account) -> Self {
        AccountDb {
            primary_id: account.id.to_string(),
            name: account.name,
            description: account.description,
            currency: account.currency,
        }
    }
}

impl TryInto<Account> for AccountDb {
    type Error = Error;

    fn try_into(self) -> Result<Account, Error> {
        let id: Uuid = Uuid::from_str(&self.primary_id)?;
        Ok(Account {
            id,
            name: self.name,
            description: self.description,
            currency: self.currency,
        })
    }
}

impl From<Transaction> for TransactionDb {
    fn from(transaction: Transaction) -> Self {
        match transaction {
            Transaction::Income {
                id,
                amount,
                to,
                category,
                date,
            } => TransactionDb {
                primary_id: id.to_string(),
                amount: amount,
                category_id: Some(category.id.to_string()),
                currency_rate: None,
                from_id: None,
                to_id: Some(to.id.to_string()),
                date: date,
            },
            Transaction::Outcome {
                id,
                amount,
                from,
                category,
                date,
            } => TransactionDb {
                primary_id: id.to_string(),
                amount: amount,
                category_id: Some(category.id.to_string()),
                currency_rate: None,
                from_id: Some(from.id.to_string()),
                to_id: None,
                date: date,
            },
            Transaction::Transfer {
                id,
                amount,
                currency_rate,
                to,
                from,
                date,
            } => TransactionDb {
                primary_id: id.to_string(),
                amount: amount,
                category_id: None,
                currency_rate: match currency_rate {
                    CurrencyRate::Empty => None,
                    CurrencyRate::Rate(rate) => Some(rate),
                },
                from_id: Some(from.id.to_string()),
                to_id: Some(to.id.to_string()),
                date: date,
            },
        }
    }
}

impl TransactionDb {
    pub(crate) fn to(
        transaction: Self,
        accounts: &[AccountDb],
        categories: &[CategoryDb],
    ) -> Result<Transaction, Error> {
        if transaction.to_id.is_some() && transaction.from_id.is_some() {
            return Ok(Transaction::Transfer {
                id: Uuid::from_str(&transaction.primary_id)?,
                amount: transaction.amount,
                currency_rate: if let Some(rate) = transaction.currency_rate {
                    CurrencyRate::Rate(rate)
                } else {
                    CurrencyRate::Empty
                },
                to: get_item_by_id::<AccountDb, Account>(transaction.to_id, accounts)?,
                from: get_item_by_id::<AccountDb, Account>(transaction.from_id, accounts)?,
                date: transaction.date,
            });
        }
        if transaction.from_id.is_some() {
            return Ok(Transaction::Outcome {
                id: Uuid::from_str(&transaction.primary_id)?,
                amount: transaction.amount,
                from: get_item_by_id::<AccountDb, Account>(transaction.from_id, accounts)?,
                category: get_item_by_id::<CategoryDb, Category>(
                    transaction.category_id,
                    categories,
                )?,
                date: transaction.date,
            });
        }
        return Ok(Transaction::Income {
            id: Uuid::from_str(&transaction.primary_id)?,
            amount: transaction.amount,
            to: get_item_by_id::<AccountDb, Account>(transaction.to_id, accounts)?,
            category: get_item_by_id::<CategoryDb, Category>(transaction.category_id, categories)?,
            date: transaction.date,
        });

        fn get_item_by_id<I, T>(id: Option<String>, items: &[I]) -> Result<T, Error>
        where
            I: DataBaseItem + TryInto<T> + Clone,
            Error: From<<I as TryInto<T>>::Error> + From<&'static str>,
        {
            let id = id.ok_or(Error::from("Id was not exist"))?;
            let item = items
                .iter()
                .find(|&el| *el.get_id() == *id)
                .ok_or(Error::from("Not found item by id"))?;
            Ok((item.clone()).try_into()?)
        }
    }
}

impl From<Category> for CategoryDb {
    fn from(category: Category) -> Self {
        CategoryDb {
            primary_id: category.id.to_string(),
            name: category.name,
            description: category.description,
        }
    }
}

impl TryInto<Category> for CategoryDb {
    type Error = Error;

    fn try_into(self) -> Result<Category, Self::Error> {
        Ok(Category {
            id: Uuid::from_str(&self.primary_id)?,
            name: self.name,
            description: self.description,
        })
    }
}
