use reqwest::blocking::{Client as HttpClient};
use std::sync::Arc;
use crate::{
    endpoint,
    model::*,
    HttpResult
};
use crate::model::music::*;
use super::make_request;
use crate::prelude::*;

pub struct Music {
    http: Arc<HttpClient>
}

impl Music {
    pub fn new(http_client: Arc<HttpClient>) -> Self {
        Self {
            http: http_client
        }
    }

    ///Get lyrics of a song specifying custom parameters
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.advanced_lyrics("despacito", false, 10) {
    ///     match res {
    ///         Ok(lyrics) => {
    ///             //do something with lyrics
    ///         },
    ///         Err(why) => {
    ///             //do something with the <MusicError> struct
    ///         }
    ///     }
    /// }
    /// ```
    pub fn advanced_lyrics(&self, query: impl ToString, text_only: bool,
                                 limit: u32) -> reqwest::Result<Lyrics> {
        if query.to_string().is_empty() { panic!("Query param cannot be empty") }

        self.http.clone().get(endpoint("/lyrics/search").as_str())
            .query(&[("q", query.to_string())])
            .query(&[("text_only", text_only)])
            .query(&[("limit", limit)])
            .send()?
            .json::<Lyrics>()
    }

    ///Get lyrics of a song
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.lyrics("despacito") {
    ///     match res {
    ///         Ok(lyrics) => {
    ///             //do something with lyrics
    ///         },
    ///         Err(why) => {
    ///             //do something with the <MusicError> struct
    ///         }
    ///     }
    /// }
    /// ```
    pub fn lyrics(&self, query: impl ToString) -> reqwest::Result<Lyrics> {
        self.advanced_lyrics(query, false, 10)
    }

    ///Get recommendations of songs with given query specifying custom parameters
    ///
    /// **You need a premium plan to use this endpoint**
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.advanced_recommendations(
    ///         ProviderType::YoutubeTitles(vec![String::from("despacito")]), None, None, None) {
    ///     match res {
    ///         Ok(recommendations) => {
    ///             //do something with recommendations
    ///         },
    ///         Err(why) => {
    ///             //do something with the <MusicError> struct
    ///         }
    ///     }
    // }
    /// ```
    pub fn advanced_recommendations(&self, tracks: ProviderType, youtube_token: Option<String>, limit: Option<u32>, recommend_type: Option<String>) -> HttpResult<MusicRecommendationsResponse, MusicError>{
        let track_vec = match &tracks {
            ProviderType::Youtube(t) => t.clone(),
            ProviderType::YoutubeIDs(t) => t.clone(),
            ProviderType::YoutubeTitles(t) => t.clone(),
            ProviderType::SpotifyIDs(t) => t.clone()
        };

        if track_vec.len() < 1 { panic!("Vector contents cannot be empty") }

        let payload = MusicRecommendations {
            tracks: track_vec,
            provider: tracks.to_string(),
            youtube_token,
            limit,
            recommend_type
        };

        let builder = self.http.clone().post(endpoint("/music/recommendations").as_str())
            .json(&payload);

        make_request::<MusicRecommendationsResponse, MusicError>(builder)
    }

    ///Get recommendations of songs with given query
    ///
    /// **You need a premium plan to use this endpoint**
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.recommendations(
    ///         ProviderType::YoutubeTitles(vec![String::from("despacito")])) {
    ///     match res {
    ///         Ok(recommendations) => {
    ///             //do something with recommendations
    ///         },
    ///         Err(why) => {
    ///             //do something with the <MusicError> struct
    ///         }
    ///     }
    // }
    /// ```
    pub fn recommendations(&self, tracks: ProviderType) -> HttpResult<MusicRecommendationsResponse, MusicError> {
        self.advanced_recommendations(tracks, None, None, None)
    }
}