use actix_web::{web, App, HttpServer};

use moko_app::controllers::file_controller;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
                           .service(file_controller::save_file))
                           .bind("127.0.0.1:1234")?
                           .run()
                           .await
}
