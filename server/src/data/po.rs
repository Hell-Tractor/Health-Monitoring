use chrono::NaiveDateTime;

use super::{vo::DataVo, dto::DataDto};

#[derive(PartialEq)]
pub struct DataPo {
    pub id: i32,
    pub value: i32,
    pub time: NaiveDateTime,
    pub data_type: i8
}

impl From<DataVo> for DataPo {
    fn from(data_vo: DataVo) -> Self {
        DataPo {
            id: -1,
            value: data_vo.value,
            time: chrono::Local::now().naive_local(),
            data_type: data_vo.data_type as i8
        }
    }
}

impl Into<DataDto> for DataPo {
    fn into(self) -> DataDto {
        DataDto {
            time: self.time.to_string(),
            value: self.value
        }
    }
}