pub mod images;
pub mod model;
pub mod error;
pub mod response;

use crate::images::Images;
use reqwest::Client as HttpClient;
use std::sync::Arc;
use reqwest::header::HeaderMap;

pub struct Client {
    pub token: String,
    pub images: Images,
    pub http: Arc<HttpClient>
}

impl Client {
    fn new<T: ToString>(token: T) -> Self {
        let mut default_auth_header =  HeaderMap::new();
        default_auth_header.insert("Authorization", token.to_string().parse().expect("Cannot parse default headers"));
        let http_client = Arc::new(HttpClient::builder().default_headers(default_auth_header).build().expect("Something went wrong when creating http client"));

        Self {
            token: token.to_string(),
            images: Images::new(token.to_string(), Arc::clone(&http_client)),
            http: http_client
        }
    }
}