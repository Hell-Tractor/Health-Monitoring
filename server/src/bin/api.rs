use actix_web::{HttpServer, App, web};
use health_monitoring_server::common::SETTINGS;
use health_monitoring_server::data;

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
                .service(data::controller::get_data_warn)
        ).wrap(actix_web::middleware::Logger::default())
    }).bind(("0.0.0.0", port))?
    .run()
    .await
}