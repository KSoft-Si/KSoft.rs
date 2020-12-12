use reqwest::{Client as HttpClient};
use std::sync::Arc;
use serde::Deserialize;
use crate::ApiResponse;
use crate::HttpResult;
use crate::model::*;
use crate::{make_request, endpoint};

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
    include_map: bool) -> HttpResult<GisResponse, KumoError>{
        if location.as_ref().is_empty() { panic!("Location param cannot be empty") }
        let builder = self.http.clone().get(endpoint("/kumo/gis").as_str())
            .query(&[("q", location.as_ref())])
            .query(&[("map_zoom", map_zoom)])
            .query(&[("fast", fast), ("more", more), ("include_map", include_map)]);

        make_request::<GisResponse, KumoError>(builder).await
    }

    pub async fn geoip(&self, ip: impl ToString) -> HttpResult<GeoIPResponse, KumoError> {
        let ip_parsed = ip.to_string().parse::<std::net::Ipv4Addr>().expect("Cannot parse as ip");

        let builder = self.http.clone().get(endpoint("/kumo/geoip").as_str())
            .query(&[("ip", ip_parsed.to_string())]);

        make_request::<GeoIPResponse, KumoError>(builder).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GisResponse {
    pub error: bool,
    pub code: u16,
    pub data: GisResponseData
}

#[derive(Clone, Debug, Deserialize)]
pub struct GisResponseData {
    pub address: String,
    pub lat: f64,
    pub lon: f64,
    pub bounding_box: Vec<String>,
    #[serde(rename = "type")]
    pub gis_type: Vec<String>,
    pub map: Option<String>
}


#[derive(Clone, Debug, Deserialize)]
pub struct GeoIPResponse {
    pub error: bool,
    pub code: u16,
    pub data: GeoIPResponseData
}

#[derive(Clone, Debug, Deserialize)]
pub struct GeoIPResponseData {
    pub city: String,
    pub continent_code: String,
    pub continent_name: String,
    pub country_code: String,
    pub country_name: String,
    pub dma_code: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub postal_code: String,
    pub region: String,
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