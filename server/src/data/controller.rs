use actix_web::{get, post, web, Responder};
use super::vo::{DataVo, GetDataVo};

#[post("")]
async fn insert_data(body: web::Json<DataVo>) -> impl Responder {
    format!("Inserting data: {:?}", body.value)
}

#[get("")]
async fn retrieve_data(query: web::Query<GetDataVo>) -> impl Responder {
    format!("Retrieving data: {}, {}", query.page, query.page_size)
}