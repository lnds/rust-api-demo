use actix_web::{web, Responder, HttpResponse};
use rand::{thread_rng, Rng};
use serde::Deserialize;

const MAX_DATA: usize = 20_000_000;

pub struct AppState {
    app_data: Vec<u8>,
}

pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let random_id : usize = thread_rng().gen_range(0, MAX_DATA);
    HttpResponse::Ok().body(format!("Random data, id = {}, value = {}", random_id, data.app_data[random_id]))
}

#[derive(Deserialize)]
pub struct ApiParams {
    id: usize,
}


pub async fn api(param: web::Path<ApiParams>, data: web::Data<AppState>) -> impl Responder {
    if param.id < MAX_DATA {
        HttpResponse::Ok().body(format!("Data for {} is {}", param.id, data.app_data[param.id]))
    } else {
        HttpResponse::NotFound().body(format!("no data for id: {}", param.id))
    }
}

impl AppState {
    pub fn load_data(_filename: &str) -> Self {
        let mut result = AppState{
            app_data: vec![0;MAX_DATA]
        };
        thread_rng().fill(&mut result.app_data[..]);
        result
    }
}
