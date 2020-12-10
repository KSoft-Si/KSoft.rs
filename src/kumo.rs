use reqwest::{Client as HttpClient, Result as HttpResult};
use std::sync::Arc;
use serde::Deserialize;
use crate::ApiResponse;
use crate::model::error::*;
use crate::make_request;

pub struct Kumo {
    pub http: Arc<HttpClient>
}

impl Kumo {
    pub fn new(http_client: Arc<HttpClient>) -> Self {
        Self {
            http: http_client
        }
    }

    pub async fn gis<S: AsRef<str>>(&self,
    location: S,
    fast: bool,
    more: bool,
    map_zoom: u8,
    include_map: bool) {
        let builder = self.http.clone().get("kumo/gis")
            .query(&[("q", location.as_ref())])
            .query(&[("map_zoom", map_zoom)])
            .query(&[("fast", fast), ("more", more), ("include_map", include_map)]);


    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GisResponse {

}

#[derive(Clone, Debug, Deserialize)]
pub struct GisResponseData {

}

#[derive(Clone, Debug, Deserialize)]
pub struct GisResponseAlerts {

}

#[derive(Clone, Debug, Deserialize)]
pub struct GisResponseLocation {
    pub lat: f64,
    pub lon: f64,
    pub address: String
}