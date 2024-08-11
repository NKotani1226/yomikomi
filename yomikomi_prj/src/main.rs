mod api;
use crate::api::fetch_books;

fn main(){
    let query = "夏目漱石";

    match fetch_books(query) {
        Ok(response) => {
            for item in response.items {
                println!("Title: {}", item.volume_info.title);
                if let Some(authors) = item.volume_info.authors {
                    println!("Authors: {:?}", authors);
                }
                if let Some(description) = item.volume_info.description {
                    println!("Description: {}", description);
                }
                if let Some(image_links) = item.volume_info.image_links {
                    if let Some(thumbnail) = image_links.thumbnail {
                        println!("Thumbnail: {}", thumbnail);
                    }
                }
            }
        }
        Err(e) => {
            println!("エラー:{}",e);
        }
    }
}
