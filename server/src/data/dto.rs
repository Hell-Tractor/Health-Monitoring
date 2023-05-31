use serde::Serialize;

#[derive(Serialize)]
pub struct DataDto {
    pub time: String,
    pub value: f64
}