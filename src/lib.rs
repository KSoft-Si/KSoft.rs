pub mod model;
pub mod prelude;


#[cfg(feature = "default")]
pub use async_trait::async_trait;
#[cfg(feature = "default")]
use reqwest::{Client as HttpClient, RequestBuilder};
#[cfg(feature = "default")]
use std::sync::Arc;
#[cfg(feature = "default")]
use reqwest::header::HeaderMap;
#[cfg(feature = "default")]
use serde::de::DeserializeOwned;

#[cfg(feature = "blocking")]
pub mod blocking;
#[cfg(feature = "default")]
pub mod images;
#[cfg(feature = "default")]
pub mod bans;
#[cfg(feature = "default")]
pub mod kumo;
#[cfg(feature = "default")]
pub mod music;
#[cfg(feature = "default")]
use crate::{
    images::Images,
    bans::Bans,
    kumo::Kumo,
    music::Music,
    model::bans::BanUpdate
};

//Asynchronous client
#[cfg(feature = "default")]
pub struct Client {
    pub token: String,
    pub images: Images,
    pub bans: Bans,
    pub kumo: Kumo,
    pub music: Music,
    pub http: Arc<HttpClient>
}

#[cfg(feature = "default")]
impl Client {
    pub fn new(token: impl ToString) -> Self {
        let mut default_auth_header =  HeaderMap::new();
        default_auth_header.insert("Authorization", format!("Bearer {}", token.to_string()).parse().expect("Cannot parse default headers"));
        let http_client = Arc::new(HttpClient::builder()
            .default_headers(default_auth_header)
            .user_agent("KSoft.rs")
            .build()
            .expect("Something went wrong when creating http client"));

        Self {
            token: token.to_string(),
            images: Images::new(Arc::clone(&http_client)),
            bans: Bans::new(Arc::clone(&http_client)),
            kumo: Kumo::new(Arc::clone(&http_client)),
            music: Music::new(Arc::clone(&http_client)),
            http: http_client
        }
    }

    pub fn event_handler(&self, handler: impl EventHandler + Send + Sync + 'static ) {
        self.bans.event_handler(handler);
    }
}

#[cfg(feature = "default")]
pub(crate) async fn make_request<S: DeserializeOwned, E: DeserializeOwned>(c: RequestBuilder) -> HttpResult<S, E> {
    let response = c.send().await?;

    return match response.status().as_u16() {
        c if c == 429u16 => Err(HttpError::RateLimited),
        c if c >= 500u16 => Err(HttpError::InternalServerError(response.text().await?)),
        200u16 => {
            let data = response.json::<S>().await?;
            Ok(Ok(data))
        },
        _ => {
            let err = response.json::<E>().await?;
            Ok(Err(err))
        }
    }
}

const BASE_ENDPOINT: &str = "https://api.ksoft.si";

pub(crate) fn endpoint(to: impl AsRef<str>) -> String {
    format!("{}{}", BASE_ENDPOINT, to.as_ref())
}


pub type HttpResult<S, E> = Result<ApiResponse<S, E>, HttpError>;

/// Result renaming used to difference between an http error and an API error or unsuccessful response
pub type ApiResponse<S, E> = Result<S, E>;

#[derive(Debug)]
pub enum HttpError {
    RequestFailed(reqwest::Error),
    InternalServerError(String),
    RateLimited
}

impl From<reqwest::Error> for HttpError {
    fn from(e: reqwest::Error) -> Self {
        HttpError::RequestFailed(e)
    }
}

#[cfg(feature = "default")]
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    ///Event triggered every 5 minutes if there is any ban update
    async fn ban_updated(&self, _data: Vec<BanUpdate>) {}
}