use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub random_url: String,
    pub oauth_url: String,
    pub search_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Photo {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub urls: Urls,
    pub links: Links,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Urls {
    pub full: String,
    pub raw: String,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    pub download_location: String,
}

#[derive(Debug, Deserialize)]
pub struct ImageUrl {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
}
