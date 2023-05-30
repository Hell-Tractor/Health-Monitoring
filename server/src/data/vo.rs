use serde::Deserialize;
use super::Type;

#[derive(Deserialize)]
pub struct DataVo {
    pub data_type: Type,
    pub value: i32
}

#[derive(Deserialize)]
pub struct GetDataVo {
    pub data_type: Type,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_page_size")]
    pub page_size: i32
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    10
}