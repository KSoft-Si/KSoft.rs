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

    /// Get artist information by a given ID
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.artist(28333u64) {
    ///     match res {
    ///         Ok(artist) => {
    ///             // do something with the artist
    ///         },
    ///         Err(why) => {
    ///             // handle error
    ///         }
    ///     }
    /// }
    pub fn artist(&self, id: impl Into<u64>) -> HttpResult<Artist, MusicError> {
        let builder = self.http.clone().get(endpoint(format!("/lyrics/artist/{}/", id.into())).as_str());

        make_request::<Artist, MusicError>(builder)
    }

    /// Get album information by a given ID
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.album(88151u64) {
    ///     match res {
    ///         Ok(album) => {
    ///             // do something with the album
    ///         },
    ///         Err(why) => {
    ///             // handle error
    ///         }
    ///     }
    /// }
    pub fn album(&self, id: impl Into<u64>) -> HttpResult<Album, MusicError> {
        let builder = self.http.clone().get(endpoint(format!("/lyrics/album/{}/", id.into())).as_str());

        make_request::<Album, MusicError>(builder)
    }

    /// Get album information by a given ID
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.track(2656006u64) {
    ///     match res {
    ///         Ok(track) => {
    ///             // do something with the track
    ///         },
    ///         Err(why) => {
    ///             // handle error
    ///         }
    ///     }
    /// }
    pub fn track(&self, id: impl Into<u64>) -> HttpResult<Track, MusicError> {
        let builder = self.http.clone().get(endpoint(format!("/lyrics/track/{}/", id.into())).as_str());

        make_request::<Track, MusicError>(builder)
    }
}