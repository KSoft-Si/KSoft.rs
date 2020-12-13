use reqwest::{Client as HttpClient};
use std::sync::Arc;
use serde::Deserialize;
use crate::{
    make_request,
    endpoint,
    model::*,
    HttpResult
};

pub struct Kumo {
    pub http: Arc<HttpClient>
}

impl Kumo {
    pub fn new(http_client: Arc<HttpClient>) -> Self {
        Self {
            http: http_client
        }
    }

    ///Get data from a given IP address
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.kumo.geoip("AmazingNonExistingIP").await {
    ///     match res {
    ///         Ok(ip) => {
    ///             //do something with ip info
    ///         },
    ///         Err(why) => {
    ///             //do something with the <KumoError> struct
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn geoip(&self, ip: impl ToString) -> HttpResult<GeoIPResponse, KumoError> {
        let ip_parsed = ip.to_string().parse::<std::net::Ipv4Addr>().expect("Cannot parse as ip");

        let builder = self.http.clone().get(endpoint("/kumo/geoip").as_str())
            .query(&[("ip", ip_parsed.to_string())]);

        make_request::<GeoIPResponse, KumoError>(builder).await
    }

    ///Performs currency conversion
    ///
    /// # Example
    /// ```rust,ignore
    /// if let Ok(res) = client.kumo.convert_currency(120.0, "USD", "EUR").await {
    ///     match res {
    ///         Ok(conversion) => {
    ///             //do something with conversion info
    ///         },
    ///         Err(why) => {
    ///             //do something with the <KumoError> struct
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn convert_currency<C: ToString>(&self, value: f64, from: C, to: C) -> HttpResult<CurrencyConversionResponse, KumoError> {
        let builder = self.http.clone().get(endpoint("/kumo/currency").as_str())
            .query(&[("from", from.to_string()), ("to", to.to_string()), ("value", value.to_string())]);

        make_request::<CurrencyConversionResponse, KumoError>(builder).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GeoIPResponse {
    pub error: bool,
    pub code: u16,
    pub data: GeoIPResponseData
}

#[derive(Clone, Debug, Deserialize)]
pub struct GeoIPResponseData {
    pub city: Option<String>,
    pub continent_code: String,
    pub continent_name: String,
    pub country_code: String,
    pub country_name: String,
    pub dma_code: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub postal_code: Option<String>,
    pub region: Option<String>,
    pub time_zone: String,
    pub apis: GeoIPResponseApis
}

#[derive(Clone, Debug, Deserialize)]
pub struct GeoIPResponseApis {
    pub weather: String,
    pub gis: String,
    #[serde(rename = "openstreetmap")]
    pub open_street_map: String,
    #[serde(rename = "googlemaps")]
    pub google_maps: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct CurrencyConversionResponse {
    pub value: f64,
    pub pretty: String
}