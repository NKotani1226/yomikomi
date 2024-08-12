use reqwest::Client;
use std::error::Error;
use crate::model::GoogleBooksResponse;

pub async fn fetch_books(query: &str) -> Result<GoogleBooksResponse, Box<dyn Error>> {
    let url = format!(
        "https://www.googleapis.com/books/v1/volumes?q={}&maxResults=1",
        query
    );

    let client = Client::new();
    let response = client.get(&url).send().await?.json::<GoogleBooksResponse>().await?;
    
    Ok(response)
}
