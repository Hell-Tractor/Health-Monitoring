use actix_web::{HttpServer, App, web};

mod data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/data")
                .service(data::controller::insert_data)
                .service(data::controller::retrieve_data)
        )
    }).bind(("127.0.0.1", 9999))?
    .run()
    .await
}