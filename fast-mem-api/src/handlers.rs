use super::db::{load_data, DataBase, Record};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, RwLock};

pub struct AppState {
    app_data: RwLock<DataBase>,
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
        }
    }

    fn len(&self) -> usize {
        self.app_data.read().unwrap().len()
    }

    fn access(&self, id: &usize) -> Option<Record> {
        let mut app_data = self.app_data.write().unwrap();
        if let Some(data) = app_data.get_mut(&id) {
            data.count += 1;
            return Some((*data).clone());
        } else {
            return None;
        }
    }

    fn stats(&self) -> Stats {
        let mut total_requests = 0;
        let mut id_requests = 0;
        let app_data = &self.app_data.read().unwrap();
        for id in app_data.keys() {
            let v = &app_data[&id];
            total_requests += v.count;
            if v.count > 0 {
                id_requests += 1;
            }
        }
        Stats {
            total_requests,
            id_requests
        }
    }
}

#[derive(Serialize)]
pub struct ResponseObj {
    id: usize,
    data: Option<Record>,
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


pub async fn stat(data: web::Data<AppState>) -> impl Responder {
    let body = serde_json::to_string(&data.stats()).unwrap();
    HttpResponse::Ok().content_type("application/json").body(body)
}
