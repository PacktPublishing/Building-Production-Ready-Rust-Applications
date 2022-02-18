use crate::api::{core_handler, order_handler, product_handler, stock_handler};
use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use futures::executor;
use log::warn;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{sync::mpsc, thread};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: SocketAddr,
}

impl ServerConfig {
    fn address(&self) -> &SocketAddr {
        &self.address
    }
}

// trait Service {
//     fn start_service() -> std::io::Result<()>;

//     fn stop_service() -> std::io::Result<()>;
// }

pub struct WebService {
    config: ServerConfig,
}

impl WebService {
    pub fn new() -> WebService {
        let config = WebService::get_config();

        WebService { config }
    }

    fn config(&self) -> &ServerConfig {
        &self.config
    }

    async fn healthcheck(req: HttpRequest) -> impl Responder {
        let name = req.match_info().get("name").unwrap_or("Monolith");
        format!("Hello {}!", &name)
    }

    pub fn get_config() -> ServerConfig {
        let config = File::open("settings.toml")
            .and_then(|mut file| {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;
                Ok(buffer)
            })
            .and_then(|buffer| {
                let server_config = toml::from_str::<ServerConfig>(&buffer)
                    .map_err(|err| std::io::Error::new(ErrorKind::Other, err));
                server_config
            })
            .map_err(|err| {
                warn!("Can't read config file: {}", err);
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
            })
            .ok();

        config.unwrap()
    }

    pub async fn start_webserver(&mut self) -> std::io::Result<()> {
        // create a channel
        let (sender, receiver) = mpsc::channel::<()>();

        let server = HttpServer::new(move || {
            App::new()
                .data(sender.clone())
                .route("/", web::get().to(WebService::healthcheck))
                .route("/stop", web::get().to(core_handler::stop))
                .service(order_handler::order_list)
                .service(
                    web::resource("/order/create")
                        .route(web::post().to(order_handler::order_create)),
                )
                .service(
                    web::resource("/order/cancel")
                        .route(web::post().to(order_handler::order_cancel)),
                )
                .service(
                    web::resource("/order/update")
                        .route(web::post().to(order_handler::order_update)),
                )
                .service(
                    web::resource("/order/fulfill")
                        .route(web::post().to(order_handler::fulfill_order)),
                )
                .route(
                    "/product/list",
                    web::get().to(product_handler::product_list),
                )
                .service(
                    web::resource("/product/create")
                        .route(web::post().to(product_handler::product_create)),
                )
                .service(
                    web::resource("/product/delete")
                        .route(web::post().to(product_handler::product_delete)),
                )
                .service(
                    web::resource("/product/update")
                        .route(web::post().to(product_handler::product_update)),
                )
                .route("/stock/list", web::get().to(stock_handler::stock_list))
                .service(
                    web::resource("/stock/create")
                        .route(web::post().to(stock_handler::stock_create)),
                )
                .service(
                    web::resource("/stock/delete")
                        .route(web::post().to(stock_handler::stock_delete)),
                )
                .service(
                    web::resource("/stock/update")
                        .route(web::post().to(stock_handler::stock_update)),
                )
                .service(
                    web::resource("/stock/increment")
                        .route(web::post().to(stock_handler::stock_increment)),
                )
        })
        .bind(self.config().address())?
        .run();

        // clone our Server handle to pass to a thread
        let srv = server.clone();

        WebService::setup_gracefulstop(srv, receiver);
        // // Create a new thread to wait for the stop signal
        // thread::spawn(move || {
        //     // blocking, while we wait for the server shutdown signal
        //     receiver.recv().unwrap();

        //     // stop server
        //     executor::block_on(srv.stop(true))
        // });

        server.await
    }

    pub fn setup_gracefulstop(srv: Server, receiver: mpsc::Receiver<()>) {
        // Create a new thread to wait for the stop signal
        thread::spawn(move || {
            // blocking, while we wait for the server shutdown signal
            receiver.recv().unwrap();

            // stop server
            executor::block_on(srv.stop(true))
        });
    }
}

// async fn example() {
//     let s = S { foo };
// }

// trait WebService {
//     fn start_service() -> Option<Service>;

//     fn stop_service() -> bool;

//     fn create_service() -> App;
// }

// #[derive(Clone)]
// pub struct WebService {
//     server: HttpServer<App, fn() -> App>,
//     name: String,
//     services: Vec<String>,
// }

// impl MainWebService {
//     pub fn new(name: &String) -> MainWebService {
//         MainWebService {
//             name: name.clone(),
//             service_type: ServiceType::Stateless,
//             args: Vec::new(),
//             dir: None,
//         }
//     }

//     /// Set the type of Microservice
//     pub fn service_type<'a>(&'a mut self, service_type: ServiceType) -> &'a mut Microservice {
//         self.service_type = service_type;
//         self
//     }

//     /// Add settings for the WebService
//     pub fn arg<'a>(&'a mut self, arg: String) -> &'a mut Microservice {
//         self.args.push(arg);
//         self
//     }

//     /// Set the working directory for the web service
//     pub fn run_location<'a>(&'a mut self, dir: String) -> &'a mut Microservice {
//         self.dir = Some(dir);
//         self
//     }

//     /// Executes the command as a child process, which is returned.
//     pub fn start(&self) -> Result<Service, ()> {
//         Ok(Service::new(&self.name))
//     }
// }
