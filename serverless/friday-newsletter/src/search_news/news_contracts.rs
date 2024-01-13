use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Source {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Article {
    // pub source: Source,
    // pub author: Option<String>,
    pub title: String,
    pub description: Option<String>,
    // pub url: String,
    // pub urlToImage: Option<String>,
    // pub publishedAt: String,
    // pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NewsResponse {
    pub status: String,
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    pub articles: Vec<Article>,
}