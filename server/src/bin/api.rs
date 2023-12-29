use actix_web::{HttpServer, App};
use health_monitoring_server::common::SETTINGS;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = SETTINGS.read().unwrap().get("server.port").unwrap();
    println!("Server started at http://0.0.0.0:{port}");
    HttpServer::new(|| {
        App::new().service(actix_files::Files::new("", "./static").index_file("index.html"))
        .wrap(actix_web::middleware::Logger::default())
    }).bind(("0.0.0.0", port))?
    .run()
    .await
}