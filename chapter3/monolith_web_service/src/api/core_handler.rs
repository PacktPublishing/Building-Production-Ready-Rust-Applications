use actix_web::{error, web, Error, HttpResponse, Responder};
use futures::StreamExt;
use std::sync::mpsc;

pub const MAX_PAYLOAD_SIZE: usize = 262_144; // max payload size is 256k

pub async fn stop(stop_server: web::Data<mpsc::Sender<()>>) -> impl Responder {
    // make request that sends message through the Sender
    stop_server.send(()).unwrap();

    HttpResponse::NoContent().finish()
}

pub async fn get_payload_bytes(mut payload: web::Payload) -> Result<web::BytesMut, Error> {
    let mut body = web::BytesMut::new();
    while let Some(data_chunk) = payload.next().await {

        // Get the data chunks
        let data_chunk = data_chunk?;

        // limit max size of in-memory payload
        if (body.len() + data_chunk.len()) > MAX_PAYLOAD_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&data_chunk);
    }

    Ok(body)
}
