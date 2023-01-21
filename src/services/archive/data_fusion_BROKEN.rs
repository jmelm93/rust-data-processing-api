//
// Endpoint Logic:
// 1. reads files from the payload
// 2. checks if it's a CSV file 
// 3. converts it to Dataframe 
// 4. stores the dataframe in a vector
// 5. converts it to json and returns
// 

//standard library
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::future::Future;
use futures_util::stream::StreamExt as _;
//datafusion
use datafusion::prelude::*;
//actix-web
use actix_web::{post, web, HttpResponse};
use actix_multipart::Multipart;
//serde(for json)
use serde_json::to_string;
use serde::{Deserialize, Serialize};
//other utils
use log::{info, debug};
use bytes::Bytes;

fn is_valid_file_type(file: &File, extension: &str) -> bool {
    let path = file.path();
    let file_extension = path.extension().unwrap().to_str().unwrap();
    file_extension == extension
}

fn to_json_response(df_vec: &Vec<Arc<DataFrame>>) -> Result<HttpResponse, HttpResponse> {
    match to_string(df_vec) {
        Ok(json_string) => {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(json_string))
        }
        Err(e) => {
            eprintln!("Error converting to json: {:?}", e);
            Err(HttpResponse::InternalServerError().finish())
        }
    }
}

async fn read_csv_file(file: File) -> Result<Arc<DataFrame>, datafusion::error::DataFusionError> {
    let mut ctx = SessionContext::new();
    let df = ctx.read_csv(file, CsvReadOptions::new()).await?;
    info!("Data loading plan created successfully!");
    Ok(Arc::new(df))
}

async fn handle_file(mut file: actix_multipart::MultipartFile) -> Result<Arc<DataFrame>, HttpResponse> {
    let file = match file.file() {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {:?}", e);
            return Err(HttpResponse::InternalServerError().finish())
        }
    };
    // check if the file is a CSV file
    if !is_valid_file_type(&file, "csv") {
        return Err(HttpResponse::BadRequest().body("Invalid file format, only CSV files are allowed."));
    }
    match read_csv_file(file).await {
        Ok(df) => Ok(df),
        Err(e) => {
            eprintln!("Error reading csv: {:?}", e);
            Err(HttpResponse::InternalServerError().finish())
        }
    }
}


#[post("/csv-upload")]
async fn csv_upload(mut payload: web::Payload) -> HttpResponse {
    // vector to hold the dataframe(s)
    let mut df_vec: Vec<Arc<DataFrame>> = vec![];

    let mut has_valid_csv_file = false;

    while let Ok(Some(mut file)) = payload.next().await {
        match handle_file(file).await {
            Ok(df) => {
                has_valid_csv_file = true;
                df_vec.push(df);
            }
            Err(e) => return e
        }
    }
    
    while let Ok(Some(mut file)) = payload.open().await {
        match handle_file(file).await {
            Ok(df) => {
                has_valid_csv_file = true;
                df_vec.push(df);
            }
            Err(e) => return e
        }
    }

    if has_valid_csv_file {
        match to_json_response(&df_vec) {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Error converting to json: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        HttpResponse::BadRequest().body("No CSV file found.")
    }

}

