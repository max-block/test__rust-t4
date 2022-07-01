use std::sync::Arc;

use actix_web::{get, web, App as HttpApp, HttpServer, Responder};

use crate::app::App;

#[get("/list_data1")]
async fn list_data1(app: web::Data<App>) -> impl Responder {
    let res = app.db.find_all_data1().await;
    serde_json::to_string(&res).unwrap()
}

#[get("/create_data1")]
async fn create_data1(app: web::Data<App>) -> impl Responder {
    app.data1_service.generate().await;
    "done"
}

#[get("/list_data2")]
async fn list_data2(app: web::Data<App>) -> impl Responder {
    let res = app.db.find_all_data2().await;
    serde_json::to_string(&res).unwrap()
}

#[get("/create_data2")]
async fn create_data2(app: web::Data<App>) -> impl Responder {
    app.data2_service.generate().await;
    "done"
}

pub async fn run_server(app: Arc<App>) -> std::io::Result<()> {
    let app = web::Data::from(app);
    HttpServer::new(move || {
        HttpApp::new()
            .app_data(web::Data::clone(&app))
            .service(list_data1)
            .service(create_data1)
            .service(list_data2)
            .service(create_data2)
    })
    .bind("localhost:3000")
    .unwrap()
    .run()
    .await
}
