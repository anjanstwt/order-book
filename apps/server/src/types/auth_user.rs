use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub image: Option<String>,
    pub is_admin: bool,
    pub exp: u64,
}

impl AuthUser {
    pub fn new(
        id: Uuid,
        email: String,
        name: String,
        image: Option<String>,
        is_admin: bool,
        exp: u64,
    ) -> Self {
        Self {
            id,
            email,
            name,
            image,
            is_admin,
            exp,
        }
    }
}
