use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use csv::ReaderBuilder;
use futures::future::{ready, Ready};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

#[derive(Clone, Serialize)]
pub struct AppData {
    value: u8,
    count: u64,
}

pub struct AppState {
    app_data: Mutex<HashMap<usize, AppData>>,
}

impl AppState {

    fn len(&self) -> usize {
       self.app_data.lock().unwrap().len()
    }

    fn access(&self, id: &usize) -> Option<AppData> {
        {
            let mut app_data = self.app_data.lock().unwrap();
            if let Some(data) = app_data.get_mut(&id) {
                data.count += 1;
                return Some((*data).clone());
            }
        }
       self.app_data.lock().unwrap().get(id).cloned()
    }

}
#[derive(Serialize)]
pub struct ResponseObj {
    id: usize,
    data: Option<AppData>,
}

impl Responder for ResponseObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        match self.data {
            Some(_) => ready(Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(body))),
            None => ready(Ok(HttpResponse::NotFound()
                .content_type("application/json")
                .body(body))),
        }
        // Create response and set content type
    }
}

pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let id: usize = thread_rng().gen_range(0, data.len());
    ResponseObj {
        id,
        data: data.access(&id),
    }
}

#[derive(Deserialize)]
pub struct ApiParams {
    id: usize,
}

pub async fn api(param: web::Path<ApiParams>, data: web::Data<AppState>) -> impl Responder {
    ResponseObj {
        id: param.id,
        data: data.access(&param.id),
    }
}

/*
pub async fn stat(data: web::Data<AppState>) -> Responder {

}
*/
#[derive(Debug, Deserialize)]
struct Record {
    id: usize,
    value: u8,
}

impl AppState {
    pub fn load_data(_filename: &str) -> AppState {
        let start = Instant::now();
        println!("loading data from {}", _filename);
        let mut data = HashMap::new();

        let mut reader = ReaderBuilder::new().from_path(_filename).unwrap();
        for row in reader.deserialize() {
            let record: Record = row.unwrap();
            data.insert(
                record.id,
                AppData {
                    value: record.value,
                    count: 0,
                },
            );
        }
        let end = Instant::now();
        println!("data loaded in {:?}", end.duration_since(start));
        AppState {
            app_data: Mutex::new(data),
        }
    }
}
