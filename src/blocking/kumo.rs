use reqwest::blocking::{Client as HttpClient};
use std::sync::Arc;
use crate::{
    endpoint,
    model::*,
    HttpResult
};
use crate::model::kumo::*;
use super::make_request;

pub struct Kumo {
    http: Arc<HttpClient>
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
    /// if let Ok(res) = client.kumo.geoip("AmazingNonExistingIP") {
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
    pub fn geoip(&self, ip: impl ToString) -> HttpResult<GeoIPResponse, KumoError> {
        let ip_parsed = ip.to_string().parse::<std::net::Ipv4Addr>().expect("Cannot parse as ip");

        let builder = self.http.get(endpoint("/kumo/geoip").as_str())
            .query(&[("ip", ip_parsed.to_string())]);

        make_request::<GeoIPResponse, KumoError>(builder)
    }

    ///Performs currency conversion
    ///
    /// # Example
    /// ```rust,ignore
    /// if let Ok(res) = client.kumo.convert_currency(120.0, "USD", "EUR") {
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
    pub fn convert_currency<C: ToString>(&self, value: f64, from: C, to: C) -> HttpResult<CurrencyConversionResponse, KumoError> {
        let builder = self.http.get(endpoint("/kumo/currency").as_str())
            .query(&[("from", from.to_string()), ("to", to.to_string()), ("value", value.to_string())]);

        make_request::<CurrencyConversionResponse, KumoError>(builder)
    }
}