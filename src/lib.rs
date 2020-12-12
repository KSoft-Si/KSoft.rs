pub mod images;
pub mod model;
pub mod bans;
pub mod kumo;
pub mod music;

use crate::{
    images::Images,
    bans::Bans,
    kumo::Kumo,
    music::Music,
    model::*
};
use reqwest::{Client as HttpClient, RequestBuilder};
use std::sync::Arc;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

pub struct Client {
    pub token: String,
    pub images: Images,
    pub bans: Bans,
    pub kumo: Kumo,
    pub music: Music,
    pub http: Arc<HttpClient>
}

impl Client {
    pub fn new<T: ToString>(token: T) -> Self {
        let mut default_auth_header =  HeaderMap::new();
        default_auth_header.insert("Authorization", format!("Bearer {}", token.to_string()).parse().expect("Cannot parse default headers"));
        let http_client = Arc::new(HttpClient::builder().default_headers(default_auth_header).build().expect("Something went wrong when creating http client"));

        Self {
            token: token.to_string(),
            images: Images::new(Arc::clone(&http_client)),
            bans: Bans::new(Arc::clone(&http_client)),
            kumo: Kumo::new(Arc::clone(&http_client)),
            music: Music::new(Arc::clone(&http_client)),
            http: http_client
        }
    }
}

pub(crate) async fn make_request<S: DeserializeOwned>(c: RequestBuilder) -> reqwest::Result<ApiResponse<S>> {
    let response = c.send().await?;

    return match response.status().as_u16() {
        200u16 => {
            let data = response.json::<S>().await?;
            Ok(ApiResponse::Success(data))
        },
        _ => {
            let err = response.json::<RawError>().await?;
            Ok(ApiResponse::Failed(err))
        }
    }
}

const BASE_ENDPOINT: &str = "https://api.ksoft.si";

pub(crate) fn endpoint(to: impl AsRef<str>) -> String {
    format!("{}{}", BASE_ENDPOINT, to.as_ref())
}

#[derive(Debug)]
pub enum ApiResponse<S> {
    Success(S),
    Failed(RawError)
}