use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
