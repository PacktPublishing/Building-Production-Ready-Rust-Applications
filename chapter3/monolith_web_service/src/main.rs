#[macro_use]
extern crate diesel;
extern crate dotenv;

mod api;
mod data;
mod models;
mod schema;
mod services;
mod web_service;

use crate::web_service::WebService;

//use actix_web::{web, App, HttpServer};

/// Runs the main web server and creates all the end points
///
/// Create a new order: /order/create
/// Cancel an Order: /order/cancel
/// Update and Order: /order/update
/// fulfill a specific Order: /order/fulfill
/// Create a product Order: /product/create
/// Delete a Product: /product/delete
/// Update a Product: /product/update
/// Create Stock: /stock/create
/// Delete Stock: /stock/delete
/// Update Stock: /stock/update
/// Increment the amount of stock: /stock/increment
///
///

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut web_service = WebService::new();
    web_service.start_webserver().await
}
