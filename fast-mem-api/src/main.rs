use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_data = handlers::AppState::load_data("../data.csv");
    HttpServer::new(move || {
        App::new()
            .data(app_data.clone())
            .route("/", web::get().to(handlers::index))
            .route("/api/{id}", web::get().to(handlers::api))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}