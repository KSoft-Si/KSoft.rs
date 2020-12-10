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
    include_map: bool) -> HttpResult<ApiResponse<GisResponse, Error404>>{
        let builder = self.http.clone().get("kumo/gis")
            .query(&[("q", location.as_ref())])
            .query(&[("map_zoom", map_zoom)])
            .query(&[("fast", fast), ("more", more), ("include_map", include_map)]);

        make_request::<GisResponse, Error404>(builder).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GisResponse {
    pub error: bool,
    pub status: u16,
    pub data: GisResponseData
}

#[derive(Clone, Debug, Deserialize)]
pub struct GisResponseData {
    pub time: String,
    pub summary: String,
    pub icon: String,
    #[serde(rename = "precipIntensity")]
    pub precip_intensity: u16,
    #[serde(rename = "precipProbability")]
    pub precip_probability: u16,
    pub temperature: f64,
    #[serde(rename = "apparentTemperature")]
    pub apparent_temperature: f64,
    #[serde(rename = "dewPoint")]
    pub dew_point: f64,
    pub humidity: f32,
    pub pressure: f32,
    #[serde(rename = "windSpeed")]
    pub wind_speed: f64,
    #[serde(rename = "windGust")]
    pub wind_gust: f64,
    #[serde(rename = "windBearing")]
    pub wind_bearing: u32,
    #[serde(rename = "cloudCover")]
    pub cloud_cover: f32,
    #[serde(rename = "uvIndex")]
    pub uv_index: u8,
    pub visibility: f32,
    pub ozone: f64,
    #[serde(rename = "sunriseTime")]
    pub sunrise_time: Option<String>,
    #[serde(rename = "sunsetTime")]
    pub sunset_time: Option<String>,
    pub icon_url: String,
    pub alerts: Vec<GisResponseAlerts>,
    pub units: String,
    pub location: GisResponseLocation
}

#[derive(Clone, Debug, Deserialize)]
pub struct GisResponseAlerts {
    pub title: String,
    pub regions: Vec<String>,
    pub severity: String,
    pub time: u64,
    pub expires: u64,
    pub description: String,
    pub uri: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct GisResponseLocation {
    pub lat: f64,
    pub lon: f64,
    pub address: String
}