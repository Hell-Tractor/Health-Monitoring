use chrono::NaiveDateTime;
use mongodb::bson::Document;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct DataDto {
    pub time: String,
    pub value: i32
}

impl From<Document> for DataDto {
    fn from(document: Document) -> Self {
        DataDto {
            time: NaiveDateTime::from_timestamp_opt(document.get_i64("time").unwrap(), 0).unwrap().to_string(),
            value: document.get_i32("value").unwrap()
        }
    }
}