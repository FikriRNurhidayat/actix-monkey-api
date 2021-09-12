use serde::Deserialize;

#[derive(Deserialize)]
pub struct MonkeyRegisterEntity {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct MonkeyLocationEntity {
    pub longitude: f32,
    pub latitude: f32,
}
