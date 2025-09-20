use crate::error::Error;
use chrono::{DateTime, Utc};
use fpcommon::{
    account::Account, category::Category, currency::CurrencyRate, transaction::Transaction,
};

pub(crate) struct TransactionDb {
    pub amount: i64,
    pub category_name: Option<String>,
    pub category_description: Option<String>,
    pub currency_rate: Option<f64>,
    pub from_name: Option<String>,
    pub from_description: Option<String>,
    pub from_currency: Option<String>,
    pub to_name: Option<String>,
    pub to_description: Option<String>,
    pub to_currency: Option<String>,
    pub date: DateTime<Utc>,
}

impl From<Transaction> for TransactionDb {
    fn from(transaction: Transaction) -> Self {
        match transaction {
            Transaction::Income {
                amount,
                to,
                category,
                date,
            } => TransactionDb {
                amount: amount,
                category_name: Some(category.name),
                category_description: None,
                currency_rate: None,
                from_name: None,
                from_description: None,
                from_currency: None,
                to_name: Some(to.name),
                to_description: None,
                to_currency: None,
                date: date,
            },
            Transaction::Outcome {
                amount,
                from,
                category,
                date,
            } => TransactionDb {
                amount: amount,
                category_name: Some(category.name),
                category_description: None,
                currency_rate: None,
                from_name: Some(from.name),
                from_description: None,
                from_currency: None,
                to_name: None,
                to_description: None,
                to_currency: None,
                date: date,
            },
            Transaction::Transfer {
                amount,
                currency_rate,
                to,
                from,
                date,
            } => TransactionDb {
                amount: amount,
                category_name: None,
                category_description: None,
                currency_rate: match currency_rate {
                    CurrencyRate::Empty => None,
                    CurrencyRate::Rate(rate) => Some(rate),
                },
                from_name: Some(from.name),
                from_description: None,
                from_currency: None,
                to_name: Some(to.name),
                to_description: None,
                to_currency: None,
                date: date,
            },
        }
    }
}

impl TryInto<Transaction> for TransactionDb {
    type Error = Error;

    fn try_into(self) -> Result<Transaction, Self::Error> {
        if self.category_name.is_none() {
            return Ok(Transaction::Transfer {
                amount: self.amount,
                currency_rate: if let Some(rate) = self.currency_rate {
                    CurrencyRate::Rate(rate)
                } else {
                    CurrencyRate::Empty
                },
                from: if let (Some(name), Some(currency)) = (self.from_name, self.from_currency) {
                    Account {
                        name,
                        description: self.from_description,
                        currency,
                    }
                } else {
                    return Err(Error::from("Error"));
                },
                to: if let (Some(name), Some(currency)) = (self.to_name, self.to_currency) {
                    Account {
                        name,
                        description: self.to_description,
                        currency,
                    }
                } else {
                    return Err(Error::from("Error"));
                },

                date: self.date,
            });
        } else if let (Some(name), Some(currency), Some(category)) =
            (self.to_name, self.to_currency, &self.category_name)
        {
            return Ok(Transaction::Income {
                amount: self.amount,
                to: Account {
                    name,
                    description: self.to_description,
                    currency,
                },
                category: Category {
                    name: String::from(category),
                    description: self.category_description,
                },
                date: self.date,
            });
        } else if let (Some(name), Some(currency), Some(category)) =
            (self.from_name, self.from_currency, self.category_name)
        {
            return Ok(Transaction::Outcome {
                amount: self.amount,
                from: Account {
                    name,
                    description: self.from_description,
                    currency,
                },
                category: Category {
                    name: category,
                    description: self.category_description,
                },
                date: self.date,
            });
        } else {
            return Err(Error::from("Error"));
        }
    }
}
