//user models
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub username: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub date_joined: chrono::DateTime<chrono::Utc>,
}

pub struct CreateNewUser {
    email: String,
    password: String,
    username: String,
}