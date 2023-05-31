use actix_web::{get, post, web, Responder};
use mysql::prelude::Queryable;
use super::vo::{DataVo, GetDataVo, GetDataSummaryVo};
use super::po::DataPo;
use crate::DB;
use crate::common::PageDto;
use crate::data::dto::DataDto;

#[post("")]
async fn insert_data(body: web::Json<DataVo>) -> actix_web::Result<impl Responder> {
    let data = DataPo::from(body.0);
    DB.write().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB write lock"))?
        .get_conn().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB connection"))?
        .query_drop(
            format!("INSERT INTO data (data_type, value, time) VALUES ({}, {}, '{}');", data.data_type as i8, data.value, data.time)
        ).map_err(|_| actix_web::error::ErrorInternalServerError("Failed to insert data"))?;
    Ok("OK")
}

#[get("")]
async fn retrieve_data(query: web::Query<GetDataVo>) -> actix_web::Result<impl Responder> {
    if query.page < 1 {
        return Err(actix_web::error::ErrorBadRequest("Page must be greater than 0"));
    }
    if query.page_size < 1 {
        return Err(actix_web::error::ErrorBadRequest("Page size must be greater than 0"));
    }

    let offset = (query.page - 1) * query.page_size;
    let result = DB.read().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB read lock"))?
        .get_conn().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB connection"))?
        .query_map(
            format!("SELECT value, time FROM data WHERE data_type = {} ORDER BY time DESC LIMIT {} OFFSET {};", query.data_type as i8, query.page_size, offset),
            |(value, time)| {
                DataDto {
                    time,
                    value
                }
            }
        ).map_err(|_| actix_web::error::ErrorInternalServerError("Failed to query data"))?;

    Ok(web::Json(PageDto {
        page: query.page,
        page_size: result.len() as i32,
        data: result
    }))
}

#[get("/{method}")]
async fn retrieve_data_summary(method: web::Path<String>, query: web::Query<GetDataSummaryVo>) -> actix_web::Result<impl Responder> {
    let method = method.into_inner();
    if method != "sum" && method != "avg" {
        return Err(actix_web::error::ErrorBadRequest("Method must be sum or avg"));
    }

    if query.page < 1 {
        return Err(actix_web::error::ErrorBadRequest("Page must be greater than 0"));
    }
    if query.page_size < 1 {
        return Err(actix_web::error::ErrorBadRequest("Page size must be greater than 0"));
    }

    let offset = (query.page - 1) * query.page_size;
    let result = DB.read().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB read lock"))?
        .get_conn().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB connection"))?
        .query_map(
            format!("SELECT min(time) as time, {}(value) as value FROM data WHERE data_type = {} AND time BETWEEN '{}' AND '{}' GROUP BY {} ORDER BY time DESC LIMIT {} OFFSET {};", method, query.data_type as i8, query.begin_time, query.end_time, query.level.to_group_sql("time"), query.page_size, offset),
            |(time, value)| {
                DataDto {
                    time,
                    value
                }
            }
        ).map_err(|_| actix_web::error::ErrorInternalServerError("Failed to query data"))?;

    Ok(web::Json(PageDto {
        page: query.page,
        page_size: result.len() as i32,
        data: result
    }))
}