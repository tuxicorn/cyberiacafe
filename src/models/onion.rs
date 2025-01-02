use serde::Serialize;

#[derive(Serialize)]
pub struct Onion {
    pub id: i32,
    pub address: String,
    pub status: String,
    pub status_nb: String,
}
