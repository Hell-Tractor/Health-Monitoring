use actix_web::{get, post, web, Responder};
use chrono::{NaiveDateTime, NaiveTime, Timelike};
use futures::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::AggregateOptions;
use super::vo::{DataVo, GetDataVo, GetDataSummaryVo, GetDataSummaryByTimeVo, GetDataWarnVo};
use super::po::DataPo;
use crate::common::{DB, SETTINGS};
use crate::common::PageDto;
use crate::data::dto::DataDto;

#[post("")]
async fn insert_data(body: web::Json<DataVo>) -> actix_web::Result<impl Responder> {
    let data = DataPo::from(body.0);
    DB.write().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB write lock"))?
        .collection("data")
        .insert_one(data, None).await.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to insert data"))?;
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
        .collection("data")
        .find(
            doc! { "data_type": (query.data_type as i32) },
            mongodb::options::FindOptions::builder()
                .sort(doc! { "time": -1 })
                .limit(Some(query.page_size as i64))
                .skip(Some(offset as u64))
                .build()
        ).await.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to query data"))?
        .filter_map(|item: Result<Document, _>| async {
            item.ok().map(|item| DataDto::from(item))
        }).collect::<Vec<_>>().await;

    Ok(web::Json(PageDto {
        page: query.page,
        page_size: result.len() as i32,
        data: result
    }))
}

#[get("/summary/{method}")]
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
    let filter = doc! {
        "data_type": query.data_type as i32,
        "time": { "$gte": query.begin_time.timestamp(), "$lte": query.end_time.timestamp() }
    };

    let pipeline = vec![
        doc! {
            "$match": filter
        },
        doc! {
            "$group": {
                "$_id": query.level.to_group_sql("$time"),
                "$time": {
                    "$min": "$time"
                },
                "value": {
                    "$sum": "$value"
                }
            }
        },
        doc! {
            "$sort": {
                "time": -1
            },
            "$limit": query.page_size,
            "$skip": offset
        }
    ];

    let result = DB.write().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB write lock"))?
        .collection::<Document>("data")
        .aggregate(pipeline, AggregateOptions::default())
        .await.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to query data"))?
        .filter_map(|item: Result<Document, _>| async {
            item.ok().map(|item| DataDto::from(item))
        }).collect::<Vec<_>>().await;

    Ok(web::Json(PageDto {
        page: query.page,
        page_size: result.len() as i32,
        data: result
    }))
}


#[get("/summary/{method}/interval/{interval}")]
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
        .collection("data")
        .find(
            doc! {
                "data_type": query.data_type as i32,
                "time": { "$gte": query.day.and_hms_opt(0, 0, 0).unwrap().timestamp(), "$lte": query.day.and_hms_opt(23, 59, 59).unwrap().timestamp() }
            },
            mongodb::options::FindOptions::builder()
                .sort(doc! { "time": -1 })
                .build()
        ).await.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to query data"))?
        .filter_map(|item: Result<Document, _>| async {
            item.ok().map(|item| (
                NaiveDateTime::from_timestamp_opt(item.get_i64("time").unwrap(), 0).unwrap().time().hour(),
                item.get_i32("value").unwrap()
            ))
        })
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
        }, |mut acc, (time, value)| async move {
            let index = (time / interval) as usize;
            acc[index].value += value;
            acc
        }).await;


    Ok(web::Json(result))
    // Ok(web::Json("OK"))
}

#[get("/warn")]
async fn get_data_warn(query: web::Query<GetDataWarnVo>) -> actix_web::Result<impl Responder> {
    let result: Option<DataDto> = None;
    let Ok(min) = SETTINGS.read().unwrap().get_int(format!("dataRange.{}.min", serde_json::to_string(&query.data_type).unwrap().trim_matches('"')).as_str()) else {
        return Ok(web::Json(result));
    };
    let Ok(max) = SETTINGS.read().unwrap().get_int(format!("dataRange.{}.max", serde_json::to_string(&query.data_type).unwrap().trim_matches('"')).as_str()) else {
        return Ok(web::Json(result));
    };
    let result = DB.read().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB read lock"))?
        .collection("data")
        .find(
            doc! {
                "data_type": query.data_type as i32,
                "$or": [
                    { "value": { "$lt": min } },
                    { "value": { "$gt": max } }
                ]
            },
            mongodb::options::FindOptions::builder()
                .sort(doc! { "time": -1 })
                .limit(Some(1))
                .build()
        ).await.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to query data"))?
        .filter_map(|item: Result<Document, _>| async {
            item.ok().map(|item| DataDto::from(item))
        }).collect::<Vec<_>>().await.get(0).map(|item| item.clone());

    Ok(web::Json(result))
}