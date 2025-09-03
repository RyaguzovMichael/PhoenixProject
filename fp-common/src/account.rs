use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Eq, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub currency: String,
}

impl Account {
    pub fn new(name: String, description: String, currency: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            currency,
        }
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
