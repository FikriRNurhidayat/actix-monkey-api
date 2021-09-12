use crate::entities::request::MonkeyRegisterEntity;
use crate::entities::response::MonkeyEntity;
use bcrypt::{DEFAULT_COST, hash};
use std::io;

pub fn create(m: &MonkeyRegisterEntity) -> Result<MonkeyEntity, io::Error> {
    let encrypted_password = match hash(&m.password, DEFAULT_COST) {
        Ok(encrypted_password) => encrypted_password,
        Err(_) => panic!("cannot encrypt password"),
    };

    Ok(MonkeyEntity{
        id: "517f9792-f199-44c4-b91b-e92139b2562b".to_string(),
        username: m.username.to_string(),
        encrypted_password,
    })
}
