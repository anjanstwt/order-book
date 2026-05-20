use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub image: Option<String>,
}

impl AuthUser {
    pub fn new(id: String, email: String, name: String, image: Option<String>) -> Self {
        Self {
            id,
            email,
            name,
            image,
        }
    }
}
