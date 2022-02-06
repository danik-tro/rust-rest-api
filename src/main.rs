use actix_web::{App, HttpServer};
use rust_web_api::config::Config;
use rust_web_api::controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Setup configuration");

    let config_file: &'static str = "config.json";
    let config = Config::from_file(config_file);

    println!("Config from {}", config_file);

    let server = HttpServer::new(move || App::new().configure(controllers::init_index))
        .bind(config.get_app_url())?;

    println!("Listening on: {}", config.get_app_url());

    server.run().await
}
