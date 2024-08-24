use reqwest::Client;
use crate::googlebooks::model::GoogleBooksResponse;
use crate::googlebooks::model::ErrorResponse;

pub async fn fetch_books(query: &str,page_no:u32) -> Result<GoogleBooksResponse, ErrorResponse> {
    let url = format!(
        "https://www.googleapis.com/books/v1/volumes?q={}&startIndex={}&orderBy=relevance",
        query,
        page_no
    );

    let client = Client::new();

    let response = client.get(&url).send().await.map_err(|e| ErrorResponse {
        error: e.to_string(),
    })?;

    let google_books_response: GoogleBooksResponse = response.json().await.map_err(|e| ErrorResponse {
        error: e.to_string(),
    })?;
    
    Ok(google_books_response)
}
