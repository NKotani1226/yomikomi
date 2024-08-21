use crate::api::fetch_books;
use actix_web::{get,web, HttpResponse, Responder,HttpServer, App};
use serde_json::json;

#[get("/")]
async fn index() -> impl Responder {

    let query = "夏目漱石";

    let items = 
    match fetch_books(query,0).await {
        Ok(response) => {
            response
        }
        Err(_e) => {
            return HttpResponse::NotFound().finish()
        }
    };
    HttpResponse::Ok()
        .body("OK")
}

#[get("/page/{page_no}")]
async fn page_index(page_no: web::Path<u32>) -> impl Responder {
    let page_no = page_no.into_inner();

    let query = "夏目漱石";

    let items = 
    match fetch_books(query,page_no).await {
        Ok(response) => {
            response
        }
        Err(_e) => {
            return HttpResponse::NotFound().finish()
        }
    };

    HttpResponse::Ok()
        .body("page")
}

pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(page_index)
            .route("/api/data",web::get().to(get_data))
    })
    .bind((addr, port))?
    .run()
    .await
}

async fn get_data() -> impl Responder {
    web::Json(json!({ "message": "Hello from Rust backend!" }))
}