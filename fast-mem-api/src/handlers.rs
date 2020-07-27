use super::db::{load_data, DataBase, Record};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::time::{Instant};
use std::sync::{RwLock, Mutex};

pub struct AppState {
    app_data: RwLock<DataBase>,
    total_request: Mutex<u64>,
    id_requests: Mutex<u64>,
}

#[derive(Serialize)]
pub struct Stats {
    total_requests: u64,
    id_requests: u64,
}


impl AppState {
    pub fn load(filename: &str) -> Self {
        let db = load_data(filename);
        AppState {
            app_data: RwLock::new(db),
            total_request: Mutex::new(0),
            id_requests: Mutex::new(0),
        }
    }

    fn len(&self) -> usize {
        self.app_data.read().unwrap().len()
    }

    fn access(&self, id: &usize) -> Option<Record> {
        let mut app_data = self.app_data.write().unwrap();
        if let Some(data) = app_data.get_mut(&id) {
            data.count += 1;
            self.inc_requests();
            if data.count == 1 {
                self.inc_id_requests();
            }
            return Some((*data).clone());
        } else {
            self.inc_requests();
            return None;
        }
    }

    fn inc_requests(&self) {
        let mut total_requests = self.total_request.lock().unwrap();
        *total_requests += 1;
    }

    fn inc_id_requests(&self) {
        let mut id_requests = self.id_requests.lock().unwrap();
        *id_requests += 1;
    }

    fn stats(&self) -> Stats {
        Stats {
            total_requests: *self.total_request.lock().unwrap(),
            id_requests: *self.id_requests.lock().unwrap(),
        }
    }
}

#[derive(Serialize)]
pub struct ResponseObj {
    id: usize,
    data: Option<Record>,
    elapsed: String
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
    let start = Instant::now();
    let id: usize = thread_rng().gen_range(0, data.len());
    ResponseObj {
        id,
        data: data.access(&id),
        elapsed: format!("{:?}", Instant::now().duration_since(start))
    }
}

#[derive(Deserialize)]
pub struct ApiParams {
    id: usize,
}

pub async fn api(param: web::Path<ApiParams>, data: web::Data<AppState>) -> impl Responder {
    let start = Instant::now();
    ResponseObj {
        id: param.id,
        data: data.access(&param.id),
        elapsed: format!("{:?}", Instant::now().duration_since(start))
    }
}



#[derive(Serialize)]
pub struct StatObj {
    data: Stats,
    elapsed: String
}


impl Responder for StatObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(body)))
    }
}


pub async fn stat(data: web::Data<AppState>) -> impl Responder {
    let start = Instant::now();
    StatObj {
        data: data.stats(),
        elapsed: format!("{:?}", Instant::now().duration_since(start))
    }
}
