use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BanError {
    pub code: u16,
    pub error: bool,
    pub exists: Option<bool>,
    pub message: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct ImageError {
    pub code: u16,
    pub error: bool,
    pub message: String,
    pub cache: Option<bool>
}

#[derive(Clone, Debug, Deserialize)]
pub struct KumoError {
    pub code: u16,
    pub error: bool,
    pub message: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct MusicError {
    pub code: u16,
    pub error: bool,
    pub message: String
}