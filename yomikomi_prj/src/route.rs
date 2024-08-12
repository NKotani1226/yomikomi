use crate::render::render_page;
use actix_web::{get, web, HttpResponse, Responder,HttpServer, App};

#[get("/")]
async fn index() -> impl Responder {
    let contents = render_page().expect("InternalError");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(contents)
}

pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind((addr, port))?
    .run()
    .await
}