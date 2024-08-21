use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize ,Deserialize)]
pub struct GoogleBooksResponse {
    pub items: Vec<Volume>,
}

#[derive(Debug, Serialize ,Deserialize)]
pub struct Volume {
    #[serde(alias = "volumeInfo")]
    pub volume_info: VolumeInfo,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct VolumeInfo {
    pub title: String,
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    #[serde(alias = "imageLinks")]
    pub image_links: Option<ImageLinks>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageLinks {
    pub thumbnail: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}