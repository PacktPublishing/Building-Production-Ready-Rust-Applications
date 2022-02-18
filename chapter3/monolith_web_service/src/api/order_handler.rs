use crate::api::core_handler::get_payload_bytes;
use crate::models::Order;
use crate::services::order_service::*;
use actix_web::{get, web, Error, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInfo {
    id: i32,
}

/// The endpoint to Complete the fulfilment of an order and updates the stock balance
/// # Arguments
///
/// * 'data' - THis is the JSON strong
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn fulfill_order(data: web::Json<OrderInfo>) -> Result<HttpResponse, Error> {
    let fulfilled = complete_fulfill_order(data.id);

    match fulfilled {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => {
            // return arbitory error for now
            let return_error = Error::from(());
            Err(return_error)
        }
    }
}

/// The endpoint to complete to get the current list of orders
/// # Arguments
///
/// * 'customer_id' - The id of the customer to get the list of oorders for
///            
/// # Return type
/// * HTTPResponse or Error
///
#[get("/order/list/{customer_id}")]
pub async fn order_list(customer_id: web::Path<i32>) -> Result<impl Responder, Error> {
    let orders = show_orders(customer_id.into_inner());
    Ok(HttpResponse::Ok().json(orders))
}

/// The endpoint to to create a new order for a perticular customer
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn order_create(payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload as bytes
    let body = get_payload_bytes(payload).await;

    match body {
        Ok(b) => {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<Order>(&b)?;

            // Call the create order service function
            let created_order = create_order(&obj);

            // Return a response
            Ok(HttpResponse::Ok().json(created_order)) // <- send response
        }
        Err(e) => Err(e),
    }
}

/// The endpoint to create a cancel an order which deletes it from the database
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
///
/// * HTTPResponse or Error
///
pub async fn order_cancel(payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload as bytes
    let body = get_payload_bytes(payload).await;

    match body {
        Ok(b) => {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<Order>(&b)?;

            // Call delete order service function
            delete_order(&obj);

            // Now return a response
            Ok(HttpResponse::Ok().json(obj)) // <- send response
        }
        Err(e) => Err(e),
    }
}

/// The endpoint to update the information on a current order
/// # Arguments
///
/// * 'payload' - this contains the JSON body data for the new order
///            
/// # Return type
/// * HTTPResponse or Error
///
pub async fn order_update(payload: web::Payload) -> Result<HttpResponse, Error> {
    // payload as bytes
    let body = get_payload_bytes(payload).await;

    match body {
        Ok(b) => {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<Order>(&b)?;

            // Call the update order service function
            update_order(&obj);

            // Now send a response
            Ok(HttpResponse::Ok().json(obj)) // <- send response
        }
        Err(e) => Err(e),
    }
}
