use serde::Serialize;

use super::data_type::Type;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataVo {
    pub data_type: Type,
    pub value: i32
}