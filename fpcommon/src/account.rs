use serde::{Deserialize, Serialize};

#[derive(Eq, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub description: Option<String>,
    pub currency: String,
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
