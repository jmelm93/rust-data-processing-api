use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{
    AppState, 
    models::entries::{CreateEntry, UpdateEntry, Entry},
};

#[get("")]
async fn get_all_entries(data: web::Data<AppState>) -> impl Responder {
    let entries = data.entries.lock().unwrap().to_vec();
    HttpResponse::Ok().json(entries)
}

#[post("")]
async fn create_entry(data: web::Data<AppState>, param_obj: web::Json<CreateEntry>) -> impl Responder {
    let mut entries = data.entries.lock().unwrap();
    let new_entry = Entry {
        id: entries.len() as i32 + 1,
        date: param_obj.date,
        entry: param_obj.entry.clone(),
    };
    entries.push(new_entry.clone());
    HttpResponse::Ok().json(new_entry)
}

#[put("/{id}")]
async fn update_entry(data: web::Data<AppState>, path: web::Path<i32>, param_obj: web::Json<UpdateEntry>) -> impl Responder {
    let id = path.into_inner();
    let mut entries = data.entries.lock().unwrap();
    let entry = entries.iter_mut().find(|entry| entry.id == id);    
    match entry {
        Some(entry) => {
            entry.entry = param_obj.entry.clone();
            HttpResponse::Ok().json(entry.clone())
        },
        None => HttpResponse::NotFound().finish(),
    }
}

#[delete("/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut entries = data.entries.lock().unwrap();
    let entry = entries.iter().position(|entry| entry.id == id);
    match entry {
        Some(entry) => {
            entries.remove(entry);
            HttpResponse::Ok().finish()
        },
        None => HttpResponse::NotFound().finish(),
    }
}