### https://stackoverflow.com/questions/75197978/rust-with-datafusion-trying-to-write-dataframe-to-json0


I a

m not sure that Datafusion is the perfect place to convert CSV string into JSON string, however here is a working version of your code:

```
#[tokio::main]
async fn main() {
    let file_name_string = "temp_file.csv".to_string();
    let csv_data_string = "heading,value\nbasic,1\ncsv,2\nhere,3".to_string();
    // Create a temporary file
    create_or_delete_csv_file(file_name_string.clone(), Some(csv_data_string), "create");
    // Create a session context
    let ctx = SessionContext::new();
    // Register the csv file
    ctx.register_csv("t1", &file_name_string, CsvReadOptions::new().has_header(false))
        .await.unwrap();
    let df = ctx.sql("SELECT * FROM t1").await.unwrap();
    // collect to vec
    let record_batches = df.collect().await.unwrap();
        // get json rows
    let json_rows = datafusion::arrow::json::writer::record_batches_to_json_rows(&record_batches[..]).unwrap();
    println!("JSON: {:?}", json_rows);
    // Delete temp csv
    create_or_delete_csv_file(file_name_string.clone(), None, "delete");
}

```