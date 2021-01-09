use serde::Deserialize;

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