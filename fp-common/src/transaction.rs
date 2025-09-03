use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{account::Account, category::Category, currency::CurrencyRate};

#[derive(Serialize, Deserialize)]
pub enum Transaction {
    Income,
    Outcome,
    Transfer,
}

#[derive(Serialize, Deserialize)]
pub struct Income {
    pub id: Uuid,
    pub amout: i64,
    pub to: Account,
    pub category: Category,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Outcome {
    pub id: Uuid,
    pub amout: i64,
    pub from: Account,
    pub category: Category,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Transfer {
    pub id: Uuid,
    pub amount: i64,
    pub currency_rate: CurrencyRate,
    pub to: Account,
    pub from: Account,
    pub date: DateTime<Utc>,
}
