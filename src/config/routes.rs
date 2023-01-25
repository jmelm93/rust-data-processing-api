use actix_web::{ web };
use log::{ info }; // , warn, error, debug, trace
use crate::{
    services::entries,
    services::data_processing,
    services::data_fixed_csv,
    static_pages::home,
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg.service(
        web::scope("/api")
            .service(home::index)
            .service(
                web::scope("/entries")
                    .service(entries::get_all_entries) 
                    .service(entries::create_entry) 
                    .service(entries::update_entry) 
                    .service(entries::delete_entry) 
            )
            .service(
                web::scope("/data-processing")
                    .service(data_processing::csv_upload)
                    .service(data_fixed_csv::testing)
                    // .service(data_processing::csv_upload_get)
            ),
    );
}

