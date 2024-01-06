use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use super::{vo::DataVo, dto::DataDto};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct DataPo {
    pub id: i32,
    pub value: i32,
    pub time: i64,
    pub data_type: i8
}

impl From<DataVo> for DataPo {
    fn from(data_vo: DataVo) -> Self {
        DataPo {
            id: -1,
            value: data_vo.value,
            time: chrono::Local::now().naive_local().timestamp(),
            data_type: data_vo.data_type as i8
        }
    }
}

impl Into<DataDto> for DataPo {
    fn into(self) -> DataDto {
        DataDto {
            time: NaiveDateTime::from_timestamp_opt(self.time, 0).unwrap().to_string(),
            value: self.value
        }
    }
}