use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct DataDto {
    pub time: String,
    pub value: i32
}