use serde::Serialize;

#[derive(Serialize)]
pub struct Order {
    pub id: u32,
    pub status: String,
    pub source: String,
    pub destination: String,
}