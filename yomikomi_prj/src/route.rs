use crate::render::render_page;
use crate::api::fetch_books;
use actix_web::{get, web, HttpResponse, Responder,HttpServer, App};

#[get("/")]
async fn index() -> impl Responder {

    let query = "夏目";

    let items = 
    match fetch_books(query).await {
        Ok(response) => {
            response
        }
        Err(e) => {
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
            .service(index)
    })
    .bind((addr, port))?
    .run()
    .await
}

#[cfg(test)]
#[allow(non_snake_case)]
mod fetch_booksテスト {

    use super::*;

    #[tokio::test]
    async fn test_fetch_books() {
        let _items =  fetch_books("夏目漱石").await;
        println!("{:?}",_items);
    }
}