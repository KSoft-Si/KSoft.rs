use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use crate::{HttpError, HttpResult};
use reqwest::blocking::{Client as HttpClient, RequestBuilder};

pub mod images;
pub mod bans;
pub mod kumo;
pub mod music;
use self::{
    images::Images,
    bans::Bans,
    kumo::Kumo,
    music::Music,
    super::model::bans::BanUpdate
};

pub struct Client {
    pub token: String,
    pub images: Images,
    pub bans: Bans,
    pub kumo: Kumo,
    pub music: Music,
    pub http: HttpClient
}

impl Client {
    pub fn new(token: impl ToString) -> Self {
        let mut default_auth_header = HeaderMap::new();
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
    /// use ksoft::blocking::{Client, EventHandler};
    /// use ksoft::model::bans::BanUpdate;
    ///
    /// fn main() {
    ///     let client = Client::new(std::env::var("KSOFT_TOKEN").expect("KSoft token not found"));
    ///     client.event_handler(Handler);
    /// }
    ///
    /// struct Handler;
    ///
    ///
    /// impl EventHandler for Handler {
    ///     fn ban_updated(&self, data: Vec<BanUpdate>) {
    ///         println!("Ban update received: {:#?}", data);
    ///     }
    /// }
    pub fn event_handler(&self, handler: impl EventHandler + Send + Sync + 'static ) {
        self.bans.event_handler(handler);
    }
}

pub(self) fn make_request<S: DeserializeOwned, E: DeserializeOwned>(c: RequestBuilder) -> HttpResult<S, E> {
    let response = c.send()?;

    return match response.status().as_u16() {
        c if c == 429u16 => Err(HttpError::RateLimited),
        c if c >= 500u16 => Err(HttpError::InternalServerError(response.text()?)),
        200u16 => {
            let data = response.json::<S>()?;
            Ok(Ok(data))
        },
        _ => {
            let err = response.json::<E>()?;
            Ok(Err(err))
        }
    }
}

pub trait EventHandler: Send + Sync + 'static {
    ///Event triggered every 5 minutes if there is any ban update
    fn ban_updated(&self, _data: Vec<BanUpdate>) {}
}
