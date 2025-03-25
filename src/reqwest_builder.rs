use reqwest::header::HeaderMap;
use secrecy::{ExposeSecret, SecretString};

pub fn get_builder(
    personal_access_token: &Option<SecretString>,
) -> anyhow::Result<reqwest::blocking::ClientBuilder> {
    let mut header_map = HeaderMap::new();
    if let Some(token) = personal_access_token {
        header_map.insert(
            "Authorization",
            format!("Bearer {}", token.expose_secret()).parse()?,
        );
    };
    Ok(reqwest::blocking::ClientBuilder::new()
        .user_agent("Github-Release-Downloader")
        .default_headers(header_map))
}
