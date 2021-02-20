use serde::Deserialize;

macro_rules! unsuccessful_response {
    ($($s: ident,)*) => {
        $(
            impl std::fmt::Display for $s {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}: {}", self.code, self.message)
                }
            }

            impl std::error::Error for $s {}
        )*
    }
}

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

unsuccessful_response! {
    BanError,
    ImageError,
    KumoError,
    MusicError,
}