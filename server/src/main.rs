use actix_web::{HttpServer, App, web};
use std::sync::RwLock;
use config::Config;

mod data;
mod common;

lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(
        Config::builder()
        .add_source(config::File::with_name("config.yaml"))
        .add_source(config::Environment::default().separator("_"))
        .build().unwrap()
    );

    static ref DB: RwLock<mysql::Pool> = {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = SETTINGS.read().unwrap().get("server.port").unwrap();
    println!("Server started at http://0.0.0.0:{port}");
    HttpServer::new(|| {
        App::new().service(
            web::scope("/data")
                .service(data::controller::insert_data)
                .service(data::controller::retrieve_data)
                .service(data::controller::retrieve_data_summary)
                .service(data::controller::get_data_group_by_hour)
        ).wrap(actix_web::middleware::Logger::default())
    }).bind(("0.0.0.0", port))?
    .run()
    .await
}