use serde::Serialize;

#[derive(Serialize)]
pub struct DataDto {
    pub time: String,
    pub value: i32
}