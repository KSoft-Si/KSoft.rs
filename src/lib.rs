pub mod model;
pub mod prelude;


#[cfg(feature = "default")]
use async_trait::async_trait;
#[cfg(feature = "default")]
use reqwest::{Client as HttpClient, RequestBuilder};
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
#[cfg(feature = "serenity")]
use typemap_rev::TypeMapKey;

use std::{
    error::Error,
    fmt::{
        Display, Formatter, Result as FmtResult
    }
};

//Asynchronous client
#[cfg(feature = "default")]
pub struct Client {
    pub token: String,
    pub images: Images,
    pub bans: Bans,
    pub kumo: Kumo,
    pub music: Music,
    pub http: HttpClient
}

#[cfg(feature = "default")]
impl Client {
    pub fn new(token: impl ToString) -> Self {
        let mut default_auth_header =  HeaderMap::new();
        default_auth_header.insert("Authorization", format!("Bearer {}", token.to_string()).parse().expect("Cannot parse default headers"));
        let http_client = HttpClient::builder()
            .default_headers(default_auth_header)
            .user_agent("KSoft.rs")
            .build()
            .expect("Something went wrong when creating http client");

        Self {
            token: token.to_string(),
            images: Images::new(http_client.clone()),
            bans: Bans::new(http_client.clone()),
            kumo: Kumo::new(http_client.clone()),
            music: Music::new(http_client.clone()),
            http: http_client
        }
    }

    /// Sets the event handler
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use ksoft::{Client, EventHandler};
    /// use ksoft::model::bans::BanUpdate;
    /// use ksoft::prelude::async_trait;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(std::env::var("KSOFT_TOKEN").expect("KSoft token not found"));
    ///     client.event_handler(Handler);
    /// }
    ///
    /// struct Handler;
    ///
    /// #[async_trait]
    /// impl EventHandler for Handler {
    ///     async fn ban_updated(&self, data: Vec<BanUpdate>) {
    ///         println!("Ban update received: {:#?}", data);
    ///     }
    /// }
    pub fn event_handler(&self, handler: impl EventHandler + Send + Sync + 'static ) {
        self.bans.event_handler(handler);
    }
}

#[cfg(feature = "serenity")]
impl TypeMapKey for Client {
    type Value = std::sync::Arc<Self>;
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

/// KSoft.rs base http response, not all methods return this
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

impl Error for HttpError {}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::RequestFailed(why) => write!(f, "Request failed: {}", why.to_string()),
            Self::InternalServerError(why) => write!(f, "Internal server error: {}", why),
            Self::RateLimited => write!(f, "KSoft server responded with code 429 (Ratelimited)")
        }
    }
}

#[cfg(feature = "default")]
#[async_trait]
pub trait EventHandler: Send + Sync + 'static {
    ///Event triggered every 5 minutes if there is any ban update
    async fn ban_updated(&self, _data: Vec<BanUpdate>) {}
}