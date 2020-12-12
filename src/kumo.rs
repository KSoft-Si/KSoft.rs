use reqwest::{Client as HttpClient, Result as HttpResult};
use std::sync::Arc;
use serde::Deserialize;
use crate::ApiResponse;
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
    include_map: bool) -> HttpResult<ApiResponse<GisResponse>>{
        let builder = self.http.clone().get(endpoint("/kumo/gis").as_str())
            .query(&[("q", location.as_ref())])
            .query(&[("map_zoom", map_zoom)])
            .query(&[("fast", fast), ("more", more), ("include_map", include_map)]);

        make_request::<GisResponse>(builder).await
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