// actix examples: https://github.com/actix/examples/
use actix_web::{
    web, 
    middleware,
    App, 
    HttpServer
};

// import from standard library
// use std::sync::Mutex; // https://doc.rust-lang.org/std/sync/struct.Mutex.html
use std:: {
    sync::Mutex, // https://doc.rust-lang.org/std/sync/struct.Mutex.html
    io::Result,
};

//logging
use log::{ info }; // , warn, error, debug, trace

// import all custom modules needed around the app
// importing here gives access to all modules in the app
mod config;
mod services;
mod models;
mod static_pages;

// use specific mods
use models::entries::{ Entry };
use config::routes::{ config_services };

// application state
struct AppState {
    entries: Mutex<Vec<Entry>>,
    // csv_data: Mutex<Vec<Entry>>,
}

#[actix_web::main]
// async fn main() -> std::io::Result<()> {
async fn main() -> Result<()> {
    // set the log level
    std::env::set_var("RUST_LOG", "info");

    // initialize logger
    env_logger::init();

    // bind context
    let bind_context = ("127.0.0.1", 8082);

    // log the bind context
    info!("Binding to: {:?}", bind_context);
    
    // create state
    let app_data = web::Data::new(AppState {
        entries: Mutex::new(vec![])
    });

    // start the server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // enable logging
            .app_data(app_data.clone())
            .configure(config_services)
    })
    .bind(bind_context)?
    .run()
    .await
}