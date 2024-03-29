use std::sync::RwLock;
use chrono::Timelike;
use rand::Rng;

use config::Config;
use lazy_static::lazy_static;

mod data_type;
mod vo;

struct DataConfig {
    data_type: data_type::Type,
    min: i32,
    max: i32,
    begin_hour: u32,
    end_hour: u32
}

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(
        Config::builder()
        .add_source(config::File::with_name("config.yaml"))
        .build().unwrap()
    );
}

async fn send(data: vo::DataVo) -> Result<(), Box<dyn std::error::Error>> {
    let url = SETTINGS.read().unwrap().get_string("server.url").unwrap();
    let client = reqwest::Client::new();
    let res = client.post(url)
        .json(&data)
        .send()
        .await?;
    println!("Response: {:?}", res);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sending_data_types = SETTINGS.read().unwrap().get_array("data").unwrap()
        .into_iter().map(|v| {
            let v = v.into_table().unwrap();
            DataConfig {
                data_type: v.get("dataType").unwrap().into(),
                min: v.get("min").unwrap().clone().into_int().unwrap() as i32,
                max: v.get("max").unwrap().clone().into_int().unwrap() as i32,
                begin_hour: v.get("beginHour").unwrap_or(&config::Value::new(None, config::ValueKind::I64(0))).clone().into_int().unwrap() as u32,
                end_hour: v.get("endHour").unwrap_or(&config::Value::new(None, config::ValueKind::I64(24))).clone().into_int().unwrap() as u32
            }
        }).collect::<Vec<DataConfig>>();
    let interval = SETTINGS.read().unwrap().get_int("interval").unwrap() as u64;
    let mut rng = rand::thread_rng();
    loop {
        let now = chrono::Local::now().naive_local().hour();
        sending_data_types.iter().for_each(|data| {
            if data.begin_hour > now || data.end_hour < now {
                return;
            }
            let value = rng.gen_range(data.min..data.max);
            let data = vo::DataVo {
                data_type: data.data_type,
                value
            };
            tokio::spawn(async move {
                send(data).await.unwrap();
            });
        });
        tokio::time::sleep(std::time::Duration::from_millis(interval)).await;
    }
}
