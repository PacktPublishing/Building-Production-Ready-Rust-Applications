use crate::data::products::*;
use crate::data::*;
use crate::handlers::core_handler::get_payload_bytes;
use crate::models::Product;
use actix_web::{web, Error, HttpResponse, Responder};
use serde_json;

/// The endpoint to get a current list of all products
/// # Arguments
///
///            
/// # Return type
/// * Responder trait or Error
///
pub async fn product_list() -> Result<impl Responder, Error> {
    let products = show_products();
    Ok(HttpResponse::Ok().json(products))
}

/// The endpoint to create a new product
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new product
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn product_create(payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload as bytes
    let body = get_payload_bytes(payload).await;

    match body {
        Ok(b) => {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<Product>(&b)?;
            println!("Success");

            let connection = get_connection();
            let created_product = create_product(&connection, &obj);

            //show_posts(false);

            Ok(HttpResponse::Ok().json(created_product)) // <- send response
        }
        Err(e) => Err(e),
    }
}

/// The endpoint to delete a new product
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new product to delete
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn product_delete(payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload as bytes
    let body = get_payload_bytes(payload).await;

    match body {
        Ok(b) => {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<Product>(&b)?;

            // Delete Order
            let connection = get_connection();
            delete_product(&connection, &obj);

            println!("Success");
            Ok(HttpResponse::Ok().json(obj)) // <- send response
        }
        Err(e) => Err(e),
    }
}

/// The endpoint to update a new product
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new product to update
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn product_update(payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload as bytes
    let body = get_payload_bytes(payload).await;

    match body {
        Ok(b) => {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<Product>(&b)?;

            // Update Order
            let connection = get_connection();
            update_product(&connection, &obj);

            println!("Success");
            Ok(HttpResponse::Ok().json(obj)) // <- send response
        }
        Err(e) => Err(e),
    }
}
