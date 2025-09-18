 use std::sync::{Arc, Mutex};

use actix_web::{web::Data, App, HttpServer};

use crate::{orderbook::Orderbook, routes::{cancel_order, create_order, get_depth}};
pub mod routes;
pub mod input;
pub mod output;
pub mod orderbook;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let orderbook = Arc::new(Mutex::new(Orderbook::new()));
    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(orderbook.clone()))
        .service(create_order)
        .service(cancel_order)
        .service(get_depth)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}