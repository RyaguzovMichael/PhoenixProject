use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum CurrencyRate {
    Empty,
    Rate(f64),
}
