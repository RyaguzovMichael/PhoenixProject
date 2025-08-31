use uuid::Uuid;

use crate::{account::Account, category::Category, currency::CurrencyRate};

pub enum Transaction {
    Income,
    Outcome,
    Transfer,
}

pub struct Income {
    pub id: Uuid,
    pub amout: i64,
    pub to: Account,
    pub category: Category,
}

pub struct Outcome {
    pub id: Uuid,
    pub amout: i64,
    pub from: Account,
    pub category: Category,
}

pub struct Transfer {
    pub id: Uuid,
    pub amount: i64,
    pub currency_rate: CurrencyRate,
    pub to: Account,
    pub from: Account,
}
