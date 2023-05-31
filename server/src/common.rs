use serde::Serialize;

#[derive(Serialize)]
pub struct PageDto<T> {
    pub page: i32,
    pub page_size: i32,
    pub data: Vec<T>
}