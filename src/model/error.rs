use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Error404 {
    pub code: Option<u16>,
    pub error: Option<bool>,
    pub exists: Option<bool>,
    pub message: Option<String>,
    pub cache: Option<bool>,
    pub total: Option<i32>,
    pub took: Option<i32>,
    pub data: Option<Vec<crate::music::Lyrics>>,
    pub voted: Option<bool>
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawError {
    pub code: Option<u16>,
    pub error: Option<bool>,
    pub exists: Option<bool>,
    pub details: Option<String>,
    pub message: Option<String>,
    pub cache: Option<bool>,
    pub total: Option<i32>,
    pub took: Option<i32>,
    pub data: Option<Vec<crate::music::Lyrics>>,
    pub voted: Option<bool>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Error400 {
    pub code: u16,
    pub error: bool,
    pub message: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Error409 {
    pub code: u16,
    pub error: bool,
    pub exists: bool,
    pub message: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Error429 {
    pub details: String
}