use dotenv::dotenv;
use models::asset::Asset;
use std::env;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

mod api_error;
mod handlers;
mod models;
mod mp4utils;
mod resources;
mod utility;
mod vtt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let is_azure = match env::var("WEBSITE_INSTANCE_ID") {
        Ok(..) => true,
        Err(..) => false,
    };

    let host = match env::var("STREAM_HOST") {
        Ok(var) => var,
        Err(..) => "localhost".to_owned(),
    };
    let port = match env::var(match is_azure {
        true => "FUNCTIONS_CUSTOMHANDLER_PORT",
        false => "STREAM_PORT",
    }) {
        Ok(var) => var.parse::<u16>().unwrap(),
        Err(..) => 3000,
    };

    let asset = Asset::new().expect("Failed to load asset");
    let data = web::Data::new(asset);

    println!("Listening on {}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(cors)
            .app_data(web::Data::clone(&data))
            .configure(handlers::master::init_routes)
            .configure(handlers::media::init_routes)
    })
    .bind((host, port))?
    .run()
    .await
}
