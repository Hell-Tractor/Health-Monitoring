use actix_web::{HttpServer, App, web};
use std::sync::RwLock;
use config::Config;

mod data;

lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(
        Config::builder()
        .add_source(config::File::with_name("config.yaml"))
        .add_source(config::Environment::default().separator("_"))
        .build().unwrap()
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", SETTINGS.read().unwrap().get_string("mysql.password").unwrap());
    HttpServer::new(|| {
        App::new().service(
            web::scope("/data")
                .service(data::controller::insert_data)
                .service(data::controller::retrieve_data)
        )
    }).bind(("127.0.0.1", SETTINGS.read().unwrap().get("server.port").unwrap()))?
    .run()
    .await
}