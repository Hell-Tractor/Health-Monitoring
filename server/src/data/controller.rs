use actix_web::{get, post, web, Responder};
use chrono::NaiveTime;
use mysql::prelude::Queryable;
use super::vo::{DataVo, GetDataVo, GetDataSummaryVo, GetDataSummaryByTimeVo};
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
            |(time, value): (String, f64)| {
                DataDto {
                    time,
                    value: value.round() as i32
                }
            }
        ).map_err(|_| actix_web::error::ErrorInternalServerError("Failed to query data"))?;

    Ok(web::Json(PageDto {
        page: query.page,
        page_size: result.len() as i32,
        data: result
    }))
}


#[get("/{method}/{interval}")]
async fn get_data_group_by_hour(path: web::Path<(String, u32)>, query: web::Query<GetDataSummaryByTimeVo>) -> actix_web::Result<impl Responder> {
    let (method, interval) = path.into_inner();
    if method != "sum" && method != "avg" {
        return Err(actix_web::error::ErrorBadRequest("Method must be sum or avg"));
    }

    if method == "avg" {
        return Err(actix_web::error::ErrorBadRequest("Method avg is not supported yet"));
    }

    if interval < 1 {
        return Err(actix_web::error::ErrorBadRequest("Interval must be greater than 0"));
    }

    if interval > 24 {
        return Err(actix_web::error::ErrorBadRequest("Interval must be less than 25"));
    }

    let result = DB.read().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB read lock"))?
        .get_conn().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB connection"))?
        .query_map(
            format!("SELECT hour(time), value FROM data WHERE data_type = {} AND date(time) = '{}';", query.data_type as i8, query.day),
            |(time, value): (u32, i32)| {
                (time, value)
            }
        ).map_err(|_| actix_web::error::ErrorInternalServerError("Failed to query data"))?
        .into_iter()
        // gourp by every {interval} hour
        .fold({
            let mut vec = Vec::<DataDto>::new();
            for i in (0..24).step_by(interval as usize) {
                vec.push(DataDto {
                    time: query.day.and_time(NaiveTime::from_hms_opt(i, 0, 0).unwrap()).to_string(),
                    value: 0
                });
            }
            vec
        }, |mut acc, (time, value)| {
            let index = (time / interval) as usize;
            acc[index].value += value;
            acc
        });


    Ok(web::Json(result))
}