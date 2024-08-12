use yomikomi_prj::api::fetch_books;
use yomikomi_prj::route::create_app;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("launching server...");
    create_app("0.0.0.0", 8080).await
}

// fn main(){
//     let query = "夏目漱石";

//     match fetch_books(query) {
//         Ok(response) => {
//             for item in response.items {
//                 println!("Title: {}", item.volume_info.title);
//                 if let Some(authors) = item.volume_info.authors {
//                     println!("Authors: {:?}", authors);
//                 }
//                 if let Some(description) = item.volume_info.description {
//                     println!("Description: {}", description);
//                 }
//                 if let Some(image_links) = item.volume_info.image_links {
//                     if let Some(thumbnail) = image_links.thumbnail {
//                         println!("Thumbnail: {}", thumbnail);
//                     }
//                 }
//             }
//         }
//         Err(e) => {
//             println!("エラー:{}",e);
//         }
//     }
// }
