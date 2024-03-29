use std::fmt::{Display, Formatter, Result as FmtResult};

#[cfg(feature = "default")]
pub use async_trait::async_trait;

#[derive(Clone)]
pub enum ProviderType {
    Youtube(Vec<String>),
    YoutubeIDs(Vec<String>),
    YoutubeTitles(Vec<String>),
    SpotifyIDs(Vec<String>)
}

impl Display for ProviderType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ProviderType::Youtube(_) => write!(f, "youtube"),
            ProviderType::YoutubeIDs(_) => write!(f, "youtube_ids"),
            ProviderType::YoutubeTitles(_) => write!(f, "youtube_titles"),
            ProviderType::SpotifyIDs(_) => write!(f, "spotify"),
        }
    }
}

impl ProviderType {
    /// Extracts the actual vector inside provider without copying it
    pub(crate) fn extract(&mut self) -> Vec<String> {
        use std::mem::take;

        match self {
            ProviderType::Youtube(d) => take(d),
            ProviderType::YoutubeIDs(d) => take(d),
            ProviderType::YoutubeTitles(d) => take(d),
            ProviderType::SpotifyIDs(d) => take(d),
        }
    }
}

pub enum SpanType {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All
}

impl Display for SpanType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            SpanType::Hour => write!(f, "hour"),
            SpanType::Day => write!(f, "day"),
            SpanType::Week => write!(f, "week"),
            SpanType::Month => write!(f, "month"),
            SpanType::Year => write!(f, "year"),
            SpanType::All => write!(f, "all")
        }
    }
}
