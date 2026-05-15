use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Password {
    pub user: Uuid,
    pub password: String,
    pub salt: String,
}