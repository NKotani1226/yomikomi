use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct GoogleBooksResponse {
    pub items: Vec<Volume>,
}

#[derive(Debug, Deserialize)]
pub struct Volume {
    #[serde(alias = "volumeInfo")]
    pub volume_info: VolumeInfo,
}

#[derive(Debug, Deserialize)]
pub struct VolumeInfo {
    pub title: String,
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    #[serde(alias = "imageLinks")]
    pub image_links: Option<ImageLinks>,
}

#[derive(Debug, Deserialize)]
pub struct ImageLinks {
    pub thumbnail: Option<String>,
}

pub fn fetch_books(query: &str) -> Result<GoogleBooksResponse, Box<dyn Error>> {
    let url = format!(
        "https://www.googleapis.com/books/v1/volumes?q={}&maxResults=1",
        query
    );

    let client = Client::new();
    let response = client.get(&url).send()?.json::<GoogleBooksResponse>()?;
    
    Ok(response)
}
