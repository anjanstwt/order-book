use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub image: Option<String>,
}

impl AuthUser {
    pub fn new(id: Uuid, email: String, name: String, image: Option<String>) -> Self {
        Self {
            id,
            email,
            name,
            image,
        }
    }
}
