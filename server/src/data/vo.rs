use chrono::NaiveDateTime;
use serde::Deserialize;
use super::Type;
use crate::SETTINGS;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataVo {
    pub data_type: Type,
    pub value: i32
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDataVo {
    pub data_type: Type,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_page_size")]
    pub page_size: i32
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SummaryLevel {
    Day,
    Week,
    Month,
    Year
}

impl SummaryLevel {
    pub fn to_group_sql(&self, property: &str) -> String {
        match self {
            SummaryLevel::Day => format!("DATE({property})"),
            SummaryLevel::Week => format!("YEAR({property}), WEEK({property})"),
            SummaryLevel::Month => format!("YEAR({property}), MONTH({property})"),
            SummaryLevel::Year => format!("YEAR({property})")
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDataSummaryVo {
    pub data_type: Type,
    pub level: SummaryLevel,
    #[serde(default = "default_begin_time")]
    pub begin_time: NaiveDateTime,
    #[serde(default = "default_end_time")]
    pub end_time: NaiveDateTime,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_page_size")]
    pub page_size: i32
}

fn default_page() -> i32 {
    SETTINGS.read().unwrap().get("query.default_page").unwrap()
}

fn default_page_size() -> i32 {
    SETTINGS.read().unwrap().get("query.default_page_size").unwrap()
}

fn default_end_time() -> NaiveDateTime {
    chrono::Local::now().naive_local()
}

fn default_begin_time() -> NaiveDateTime {
    default_end_time() - chrono::Duration::days(1)
}