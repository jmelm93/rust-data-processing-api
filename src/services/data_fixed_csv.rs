// datafusion examples: https://github.com/apache/arrow-datafusion/tree/master/datafusion-examples/examples
// datafusion docs: https://arrow.apache.org/datafusion/
use datafusion::prelude::*;
// use datafusion::prelude::Column;
use datafusion::arrow::datatypes::{DataType, Field, Schema};

// https://crates.io/crates/actix_extract_multipart
use actix_web::{ get, HttpResponse, error, Error }; 
// use actix_extract_multipart::{ Multipart };

use std::sync::Arc;
use std::str;
use std::fs;
use std::future::Future;
use std::ops::Deref;

// use tempfile::NamedTempFile;
use csv::Reader;
use tempfile::NamedTempFile;
use std::fs::File;
use std::io::prelude::*;

use serde::{ Deserialize };
use serde_json::to_string;

type DFResult = Result<Arc<DataFrame>, datafusion::error::DataFusionError>;

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

fn create_or_delete_csv_file(path: String, content: Option<String>, operation: &str) {
    match operation {
        "create" => {
            // Create the data to put into the csv file with headers
            fs::write(path, content).expect("Problem with writing file!");
        }
        "delete" => {
            // Delete the csv file
            fs::remove_file(path).expect("Problem with deleting file!");
        }
        _ => println!("Invalid operation"),
    }
}

fn convert_vec_u8_to_string(vec: Vec<u8>) -> impl Future<Output = Result<String, Error>> {
    async move {
        let string = str::from_utf8(&vec).map_err(|e| {
            error::ErrorBadRequest(e)
        })?;
        Ok(string.to_string())
    }
}

async fn read_csv_file_with_inferred_schema() -> DFResult {
    let csv_data_string = "heading,value\nbasic,1\ncsv,2\nhere,3".to_string();
    let file_name_string = "temp_file.csv".to_string();

    // Create a temporary file
    create_or_delete_csv_file(file_name_string.clone(), Some(csv_data_string), "create");

    // Create a session context
    let ctx = SessionContext::new();

    // Register a lazy DataFrame using the context
    let df = ctx.read_csv(file_name_string.clone(), CsvReadOptions::default()).await.expect("An error occurred while reading the CSV string");

    // Delete temp csv
    // create_or_delete_csv_file(file_name_string.clone(), None, "delete");

    // return the dataframe
    Ok(Arc::new(df)) 
}

#[get("/fixed-csv")]
async fn testing() -> HttpResponse {

    let arc_csv_df = read_csv_file_with_inferred_schema().await.expect("An error occurred while reading the CSV string (funct: read_csv_file_with_inferred_schema)");

    // have to use ".clone()" each time I want to use this ref
    let deref_df = arc_csv_df.deref();

    // print to console
    deref_df.clone().show().await.expect("An error occurred while showing the CSV DataFrame");

    // collect to vec
    let data = deref_df.clone().collect().await.expect("An error occurred while collecting the CSV DataFrame");
    // println!("Data: {:?}", data);

    // get final values from recordbatch
    // https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatch.html
    // https://users.rust-lang.org/t/how-to-use-recordbatch-in-arrow-when-using-datafusion/70057/2
    // https://github.com/apache/arrow-rs/blob/6.5.0/arrow/src/util/pretty.rs
    let data_vec = data.to_vec();

    let mut header = Vec::new();
    // let mut rows = Vec::new();

    for record_batch in data_vec {
        // get data
        for col in record_batch.columns() {
            println!("Cow: {:?}", col);
            for row in 0..col.len() {
                println!("Row: {:?}", row);
                // let value = col.as_any().downcast_ref::<StringArray>().unwrap().value(row);
                // rows.push(value);
            }
        }
        // get headers
        for field in record_batch.schema().fields() {
            header.push(field.name().to_string());
        }
    };

    println!("Header: {:?}", header);
    // println!("Rows: {:?}", rows);

    return HttpResponse::Ok().json("Complete");

}