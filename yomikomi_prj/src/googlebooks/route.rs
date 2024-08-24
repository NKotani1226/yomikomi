use crate::googlebooks::api::fetch_books;
use actix_web::{web, Responder,HttpServer, App};

pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/page/{page_no}",web::get().to(page_index))
            .route("/",web::get().to(index))
    })
    .bind((addr, port))?
    .run()
    .await
}
async fn index() -> impl Responder {

    let query = "夏目漱石";

    let result = fetch_books(query,0).await;

    match result {
        Ok(ref data) => {
            // シリアライズして出力
            println!("{}", serde_json::to_string(&data).unwrap());
        }
        Err(ref e) => {
            // エラーレスポンスをシリアライズして出力
            println!("{}", serde_json::to_string(&e).unwrap());
        }
    }

    web::Json(result)
}

async fn page_index(page_no: web::Path<u32>) -> impl Responder {
    let page_no = page_no.into_inner();

    let query = "夏目漱石";

    let result = fetch_books(query,page_no).await;

    match result {
        Ok(ref data) => {
            // シリアライズして出力
            println!("{}", serde_json::to_string(&data).unwrap());
        }
        Err(ref e) => {
            // エラーレスポンスをシリアライズして出力
            println!("{}", serde_json::to_string(&e).unwrap());
        }
    }

    web::Json(result)
}