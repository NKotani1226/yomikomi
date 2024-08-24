use crate::googlebooks::api::fetch_books;
use actix_web::{web, Responder,HttpServer, App};
use actix_cors::Cors;
pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:12000") // フロントエンドのURLを許可
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .max_age(3600)
            )
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