use reqwest::Client;
use std::error::Error;
use crate::model::GoogleBooksResponse;

pub async fn fetch_books(query: &str,page_no:u32) -> Result<GoogleBooksResponse, Box<dyn Error>> {
    let url = format!(
        "https://www.googleapis.com/books/v1/volumes?q={}&startIndex={}&orderBy=relevance",
        query,
        page_no
    );

    let client = Client::new();
    let response = client.get(&url).send().await?.json::<GoogleBooksResponse>().await?;
    
    Ok(response)
}
