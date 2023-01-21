// //
// // Endpoint Logic:
// // 1. reads files from the payload
// // 2. checks if it's a CSV file 
// // 3. converts it to Dataframe 
// // 4. stores the dataframe in a vector
// // 5. converts it to json and returns
// // 

// // datafusion examples: https://github.com/apache/arrow-datafusion/tree/master/datafusion-examples/examples
// // datafusion docs: https://arrow.apache.org/datafusion/
// use std::fs::File;
// use std::io::Write;
// use std::sync::Arc;

// use datafusion::prelude::*;

// use actix_multipart::Multipart;
// // use actix_web::{middleware, post, web, App, Error, HttpResponse, HttpServer};
// use actix::ResponseFuture;
// use actix_web::dev::Payload;
// use actix_web::http::header::ContentDisposition;
// use actix_web::http::Method;
// use actix_web::{error, multipart, Error, HttpMessage, HttpRequest, Json, Query, State};
// use bytes::BytesMut;
// use futures::{
//     future::{self, IntoFuture},
//     Future, Stream,
// };

// enum MultipartItem {
//     ProcessFile(fs::File), 
//     Sources(Vec<SourceConfig>),
// }


// fn is_valid_file_type(filename: &str, extension: &str) -> bool {
//     // confirm if filename contains the extension
//     filename.contains(extension)
// }

// // async fn 

// // async fn read_csv_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
// //     // iterate over multipart stream
// //     while let Some(mut field) = payload.try_next().await? {
// //         // A multipart/form-data stream has to contain `content_disposition`
// //         let content_disposition = field.content_disposition();

// //         let filename = content_disposition
// //             .get_filename()
// //             .unwrap()
// //             .to_string();
        
// //         // check if file is a csv
// //         if !is_valid_file_type(&filename, "csv") {
// //             return Ok(HttpResponse::BadRequest().body("Invalid file type"));
// //         }

// //         info!("File name: {}", filename);

// //         // return file to the caller
// //         return filename;
// //     }

// //     return Ok(HttpResponse::Ok().into())

// // }

// #[post("/csv-upload")]
// async fn csv_upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
//     // create local execution context
//     let ctx = SessionContext::new();

//     // pass payload to read_csv_file
//     let filename = read_csv_file(payload).await?;

//     // read csv file
//     let df = ctx.read_csv(filename, CsvReadOptions::new()).await?;
// //     // create a vector to store dataframes (vector is a growable array)
// //     let mut df_vec: Vec<Arc<DataFrame>> = Vec::new();

// //     // iterate over multipart stream
// //     while let Some(mut field) = payload.try_next().await? {
// //         // A multipart/form-data stream has to contain `content_disposition`
// //         let content_disposition = field.content_disposition();

// //         let filename = content_disposition
// //             .get_filename()
// //             .unwrap()
// //             .to_string();
        
// //         // check if file is a csv
// //         if !is_valid_file_type(&filename, "csv") {
// //             return Ok(HttpResponse::BadRequest().body("Invalid file type"));
// //         }

// //         info!("File name: {}", filename);

// //     }

// //     Ok(HttpResponse::Ok().into())
// // }

// //     // only handle single file. Get file and register it with the execution context
// //     while let Some(item) = payload.next().await {
// //         let mut field = item.unwrap();
// //         let content_type = field.content_disposition().unwrap();
// //         let filename = content_type.get_filename().unwrap();

// //         // check if file is a csv
// //         if !is_valid_file_type(filename, "csv") {
// //             return HttpResponse::BadRequest().body("Invalid file type");
// //         }

// //         // read file into a buffer
// //         let mut buffer = Vec::new();
// //         while let Some(chunk) = field.next().await {
// //             let data = chunk.unwrap();
// //             buffer.extend_from_slice(&data);
// //         }

// //         // convert buffer to file
// //         let file = File::new(filename, buffer);

// //         // register file with the execution context
// //         let df = ctx.read_csv(file, CsvReadOptions::new()).await.unwrap();
// //         df_vec.push(Arc::new(df));

// //     }

// //     // .show each dataframe in the vector
// //     for df in df_vec {
// //         df.show();
// //     }

// //     HttpResponse::Ok().body("File uploaded")

// // }



        

//     //     let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));
//     //     let mut f = web::block(|| std::fs::File::create(filepath))
//     //         .await
//     //         .unwrap();

//     //     // File::create is blocking operation, use threadpool
//     //     while let Some(chunk) = field.next().await {
//     //         let data = chunk.unwrap();
//     //         f = web::block(move || f.write_all(&data).map(|_| f))
//     //             .await
//     //             .unwrap();
//     //     }
//     // }