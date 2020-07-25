use actix_web::{web, Responder, HttpResponse};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone)]
pub struct AppState {
    app_data: HashMap<usize, u8>,
}

pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let random_id : usize = thread_rng().gen_range(0, data.app_data.len());
    HttpResponse::Ok().body(format!("Random data, id = {}, value = {}", random_id, data.app_data[&random_id]))
}

#[derive(Deserialize)]
pub struct ApiParams {
    id: usize,
}


pub async fn api(param: web::Path<ApiParams>, data: web::Data<AppState>) -> impl Responder {
    if data.app_data.contains_key(&param.id) {
        HttpResponse::Ok().body(format!("Data for {} is {}", param.id, data.app_data[&param.id]))
    } else {
        HttpResponse::NotFound().body(format!("no data for id: {}", param.id))
    }
}

#[derive(Debug, Deserialize)]
struct Record {
    id: usize,
    value: u8,
}

impl AppState {
    pub fn load_data(_filename: &str) -> AppState {
        let start = Instant::now();
        println!("loading data from {}", _filename);
        let mut data = AppState{
            app_data: HashMap::new()
        };
        let mut reader = ReaderBuilder::new().from_path(_filename).unwrap();
        for row in reader.deserialize() {

            let record : Record = row.unwrap();
            data.app_data.insert(record.id, record.value);
        }
        let end = Instant::now();
        println!("data loaded in {:?}", end.duration_since(start));
        data
    }
}
