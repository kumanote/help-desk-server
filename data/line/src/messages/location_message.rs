use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct LocationMessage {
    pub title: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}
