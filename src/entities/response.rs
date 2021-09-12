use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MonkeyEntity {
    pub id: String,
    pub username: String,
    pub encrypted_password: String,
}
