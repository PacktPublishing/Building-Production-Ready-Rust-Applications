use futures_util::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use std::{convert::Infallible, net::SocketAddr};

use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::str;
use std::sync::{Arc, Mutex};

use serde::de;

use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};

struct State(u64);

// A handler for "/" page.
async fn home_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Access the app state.
    let state = req.data::<State>().unwrap();
    println!("State value: {}", state.0);

    Ok(Response::new(Body::from("Home page")))
}

// A handler for "/users/:userId" page.
async fn user_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let user_id = req.param("userId").unwrap();
    Ok(Response::new(Body::from(format!("Hello {}", user_id))))
}

/*
data: {"order":{"line_items":[{"variant_id":447654529,"quantity":1}]}},
  type: DataType.JSON,
*/

// A handler for "/users/:userId" page.
async fn order_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // let product_name = req.param("productName").unwrap();
    // let amount = req.param("amount").unwrap();

    //deserialize(req);

    // let mapping = req.into_body().map_ok(|chunk| {
    //     chunk
    //         .iter()
    //         .map(|byte| byte.to_ascii_uppercase())
    //         .collect::<Vec<u8>>()
    // }); //.rev().cloned().collect();

    // let s = match str::from_utf8(mapping.get_ref()) {
    //     Ok(v) => v,
    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };

    //let body = Body::wrap_stream(mapping);

    let full_body = hyper::body::to_bytes(req.into_body()).await;

    // Iterate the full body in reverse order and collect into a new Vec.
    let reversed = full_body.iter().rev().cloned().collect::<Vec<u8>>();

    Ok(Response::new(Body::from(format!("Hello"))))
}

// A middleware which logs an http request.
async fn logger(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    println!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

// Define an error handler function which will accept the `routerify::Error`
// and the request information and generates an appropriate response.
async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    eprintln!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

pub fn router() -> Router<Body, Infallible> {
    // Create a router and specify the logger middleware and the handlers.
    // Here, "Middleware::pre" means we're adding a pre middleware which will be executed
    // before any route handlers.
    Router::builder()
        // Specify the state data which will be available to every route handlers,
        // error handler and middlewares.
        .data(State(100))
        .middleware(Middleware::pre(logger))
        .get("/", home_handler)
        .get("/users/:userId", user_handler)
        .post("/order/", order_handler)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}
