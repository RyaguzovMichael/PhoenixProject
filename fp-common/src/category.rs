use uuid::Uuid;

pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
