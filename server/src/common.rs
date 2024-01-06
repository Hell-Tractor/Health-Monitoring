use serde::Serialize;
use std::sync::RwLock;
use config::Config;

#[derive(Serialize)]
pub struct PageDto<T> {
    pub page: i32,
    pub page_size: i32,
    pub data: Vec<T>
}

lazy_static::lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(
        Config::builder()
        .add_source(config::File::with_name("config.yaml"))
        .add_source(config::Environment::default().separator("_"))
        .build().unwrap()
    );

    pub static ref DB: RwLock<mongodb::Database> = {
        let host = SETTINGS.read().unwrap().get_string("mongodb.host").unwrap();
        let port = SETTINGS.read().unwrap().get_string("mongodb.port").unwrap();
        let database = SETTINGS.read().unwrap().get_string("mongodb.database").unwrap();
        let client_options = mongodb::options::ClientOptions::builder().hosts(vec![format!("mongodb://{}:{}", host, port).parse().unwrap()]);
        let client = mongodb::Client::with_options(client_options.build()).unwrap();
        RwLock::new(
            client.database(database.as_str())
        )
    };
}