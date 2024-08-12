use yomikomi_prj::route::create_app;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("launching server...");
    create_app("0.0.0.0", 8080).await
}

