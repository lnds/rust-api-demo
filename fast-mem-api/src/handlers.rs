use actix_web::{Error, web, Responder, HttpRequest, HttpResponse};
use futures::future::{ready, Ready};
use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone)]
pub struct AppState {
    app_data: HashMap<usize, u8>,
}

#[derive(Serialize)]
pub struct ResponseObj {
    id: usize,
    value: Option<u8>,
}

impl Responder for ResponseObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        match self.value {
            Some(_) => ready(Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(body))),
            None => ready(Ok(HttpResponse::NotFound()
                .content_type("application/json")
                .body(body)))
        }
        // Create response and set content type

    }

}

pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let id : usize = thread_rng().gen_range(0, data.app_data.len());
    ResponseObj {
        id,
        value: data.app_data.get(&id).cloned(),
    }
}

#[derive(Deserialize)]
pub struct ApiParams {
    id: usize,
}


pub async fn api(param: web::Path<ApiParams>, data: web::Data<AppState>) -> impl Responder {
        ResponseObj {
            id: param.id,
            value: data.app_data.get(&param.id).cloned()
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
