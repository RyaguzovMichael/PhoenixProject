use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{account::Account, category::Category, currency::CurrencyRate};

#[derive(Serialize, Deserialize)]
pub enum Transaction {
    Income {
        amount: i64,
        to: Account,
        category: Category,
        date: DateTime<Utc>,
    },
    Outcome {
        amount: i64,
        from: Account,
        category: Category,
        date: DateTime<Utc>,
    },
    Transfer {
        amount: i64,
        currency_rate: CurrencyRate,
        to: Account,
        from: Account,
        date: DateTime<Utc>,
    },
}
