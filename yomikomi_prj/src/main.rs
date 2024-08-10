mod api;
use std::error::Error;
use crate::api::fetch_books;

fn main() -> Result<(), Box<dyn Error>> {
    let query = "夏目漱石";
    
    let response = fetch_books(query)?;

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
        println!("---");
    }

    Ok(())
}
