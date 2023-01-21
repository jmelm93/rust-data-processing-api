// https://elferherrera.github.io/arrow_guide/reading_parquet.html
// use arrow::{
//     record_batch::RecordBatch,
// };

// datafusion examples: https://github.com/apache/arrow-datafusion/tree/master/datafusion-examples/examples
// datafusion docs: https://arrow.apache.org/datafusion/
use datafusion::prelude::*;
use datafusion::prelude::Column;
use datafusion::arrow::datatypes::{DataType, Field, Schema};
// use datafusion::dataframe::column::Column;

// https://crates.io/crates/actix_extract_multipart
use actix_web::{ post, HttpResponse, error, Error }; 
use actix_extract_multipart::{ Multipart, File };

use std::sync::Arc;
use std::str;
use std::fs;
use std::future::Future;
use std::ops::Deref;

use serde::{ Deserialize };
use serde_json::to_string;

use log::{ info }; // , warn, error, debug, trace

// // Accepted files extensions
// const FILES_EXTENSIONS: [&str; 2] = ["csv", "txt"];


#[derive(Deserialize)]
struct FormObject {
    report_name: String,
    file_param: Option<File>,
}

struct FinalObject {
    schema: Schema,
    // columns: Vec<Column>,
    num_rows: usize,
    num_columns: usize,
}

// to allow debug logging for FinalObject
impl std::fmt::Debug for FinalObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "FinalObject {{ schema: {:?}, columns: {:?}, num_rows: {:?}, num_columns: {:?} }}",
        write!(f, "FinalObject {{ schema: {:?}, num_rows: {:?}, num_columns: {:?} }}",
                // self.schema,  self.columns, self.num_columns, self.num_rows)
                self.schema, self.num_columns, self.num_rows)
    }
}

type DFResult = Result<Arc<DataFrame>, datafusion::error::DataFusionError>;
// type DFResult = Result<DataFrame, datafusion::error::DataFusionError>;

fn convert_vec_u8_to_string(vec: Vec<u8>) -> impl Future<Output = Result<String, Error>> {
    async move {
        let string = str::from_utf8(&vec).map_err(|e| {
            error::ErrorBadRequest(e)
        })?;
        Ok(string.to_string())
    }
}

fn create_csv_file(path: String, content: String) {
    // Create the data to put into the csv file with headers
    fs::write(path, content).expect("Problem with writing file!");
}

fn delete_csv_file(path: String) {
    // Delete the csv file
    fs::remove_file(path).expect("Problem with deleting file!");
}

async fn read_csv_file_with_inferred_schema(file: &actix_extract_multipart::File, path: String) -> DFResult {
    
    // get file data as a string
    let file_data_string = convert_vec_u8_to_string(file.data().to_vec()).await.expect("An error occurred while converting the file data to string");

    // create file from filename and file_data_string
    create_csv_file(path.to_string(), file_data_string);
    
    // Create a session context
    let ctx = SessionContext::new();

    // Register a lazy DataFrame using the context
    let df = ctx.read_csv(path, CsvReadOptions::default()).await.expect("An error occurred while reading the CSV string");

    // return the dataframe
    Ok(Arc::new(df)) 
}

#[post("/csv-upload")]
async fn csv_upload(payload: Multipart::<FormObject>) -> HttpResponse {

    info!("Report Name: {}", payload.report_name);

    if payload.file_param.is_some() {
        
        let file = match payload.file_param.as_ref() {
            Some(file) => file,
            None => {
                eprintln!("File parameter is missing from payload");
                return HttpResponse::BadRequest().json("File parameter is missing from payload");
            },
        };
        
        let arc_csv_df = read_csv_file_with_inferred_schema(file, file.name().to_string()).await.expect("An error occurred while reading the CSV string (funct: read_csv_file_with_inferred_schema)");
        // have to use ".clone()" each time I want to use this ref
        let deref_df = arc_csv_df.deref();

        // print to console
        deref_df.clone().show().await.expect("An error occurred while showing the CSV DataFrame");

        // collect to vec
        let data = deref_df.clone().collect().await.expect("An error occurred while collecting the CSV DataFrame");
        // info!("Data: {:?}", data);

        // get final values from recordbatch
        // A two-dimensional batch of column-oriented data with a defined schema.
        // A RecordBatch is a two-dimensional dataset of a number of contiguous arrays, each the same length. A record batch has a schema which must match its arrays’ datatypes.
        // Record batches are a convenient unit of work for various serialization and computation functions, possibly incremental.
        // https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatch.html
        // https://users.rust-lang.org/t/how-to-use-recordbatch-in-arrow-when-using-datafusion/70057/2
        // https://github.com/apache/arrow-rs/blob/6.5.0/arrow/src/util/pretty.rs
        let data_vec = data.to_vec();

        // let serialized_data = data.serialize().expect("An error occurred while serializing the CSV DataFrame");

        let mut header = Vec::new();
        // let mut rows = Vec::new();

        for record_batch in data_vec {
            // get data
            for col in record_batch.columns() {
                info!("Cow: {:?}", col);
                for row in 0..col.len() {
                    info!("Row: {:?}", row);
                    // let value = col.as_any().downcast_ref::<StringArray>().unwrap().value(row);
                    // rows.push(value);
                }
            }
            // get headers
            for field in record_batch.schema().fields() {
                header.push(field.name().to_string());
            }
        };

        info!("Header: {:?}", header);
        // info!("Rows: {:?}", rows);


        // delete csv file
        delete_csv_file(file.name().to_string());

        return HttpResponse::Ok().json("Complete");

    } else {
        // file_param is missing from payload
        eprintln!("File parameter is missing from payload");
        return HttpResponse::BadRequest().json("File parameter is missing from payload");
    }
}

// if FILES_EXTENSIONS.contains(&file.file_type().as_str()) {
//     info!("Correct file type...");
//     return HttpResponse::Ok().json("Done");
// } 