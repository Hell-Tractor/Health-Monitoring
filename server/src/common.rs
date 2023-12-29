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

    pub static ref DB: RwLock<mysql::Pool> = {
        let user = SETTINGS.read().unwrap().get_string("mysql.user").unwrap();
        let password = SETTINGS.read().unwrap().get_string("mysql.password").unwrap();
        let host = SETTINGS.read().unwrap().get_string("mysql.host").unwrap();
        let port = SETTINGS.read().unwrap().get_string("mysql.port").unwrap();
        let database = SETTINGS.read().unwrap().get_string("mysql.database").unwrap();
        RwLock::new(
            mysql::Pool::new(
                format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, database).as_str()
            ).unwrap()
        )
    };
}