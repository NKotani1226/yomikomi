use crate::render::render_page;
use crate::api::fetch_books;
use actix_web::{get,web, HttpResponse, Responder,HttpServer, App};
use actix_files as fs;

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

    let contents = render_page(items).expect("InternalError");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(contents)
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

    let contents = render_page(items).expect("InternalError");

    HttpResponse::Ok()
        .content_type("text/html")
        .body(contents)
}

pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/assets","assets/").show_files_listing())
            .service(index)
            .service(page_index)
    })
    .bind((addr, port))?
    .run()
    .await
}
