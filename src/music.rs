use reqwest::{Client as HttpClient};
use crate::{
    make_request,
    endpoint,
    model::*,
    HttpResult
};
use crate::model::music::*;
use crate::prelude::*;

pub struct Music {
    http: HttpClient
}

impl Music {
    pub fn new(http_client: HttpClient) -> Self {
        Self {
            http: http_client
        }
    }

    ///Get lyrics of a song specifying custom parameters
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.advanced_lyrics("despacito", false, 10).await {
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
    pub async fn advanced_lyrics(&self, query: impl ToString, text_only: bool,
                                 limit: u32) -> reqwest::Result<Lyrics> {
        if query.to_string().is_empty() { panic!("Query param cannot be empty") }

        let response = self.http.get(endpoint("/lyrics/search").as_str())
            .query(&[("q", query.to_string())])
            .query(&[("text_only", text_only)])
            .query(&[("limit", limit)])
            .send()
            .await?;

        response.json::<Lyrics>().await
    }

    ///Get lyrics of a song
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.lyrics("despacito").await {
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
    pub async fn lyrics(&self, query: impl ToString) -> reqwest::Result<Lyrics> {
        self.advanced_lyrics(query, false, 10).await
    }

    ///Get recommendations of songs with given query specifying custom parameters
    ///
    /// **You need a premium plan to use this endpoint**
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.advanced_recommendations(
    ///         ProviderType::YoutubeTitles(vec![String::from("despacito")]), None, None, None).await {
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
    pub async fn advanced_recommendations(&self, mut provider: ProviderType, youtube_token: Option<String>, limit: Option<u32>, recommend_type: Option<String>) -> HttpResult<MusicRecommendationsResponse, MusicError>{
        let track_vec = provider.extract();

        if track_vec.len() < 1 { panic!("Vector contents cannot be empty") }

        let payload = MusicRecommendations {
            tracks: track_vec,
            provider: provider.to_string(),
            youtube_token,
            limit,
            recommend_type
        };

        let builder = self.http.post(endpoint("/music/recommendations").as_str())
            .json(&payload);

        make_request::<MusicRecommendationsResponse, MusicError>(builder).await
    }

    ///Get recommendations of songs with given query
    ///
    /// **You need a premium plan to use this endpoint**
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.recommendations(
    ///         ProviderType::YoutubeTitles(vec![String::from("despacito")])).await {
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
    pub async fn recommendations(&self, provider: ProviderType) -> HttpResult<MusicRecommendationsResponse, MusicError> {
        self.advanced_recommendations(provider, None, None, None).await
    }

    /// Get artist information by a given ID
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.artist(28333u64).await {
    ///     match res {
    ///         Ok(artist) => {
    ///             // do something with the artist
    ///         },
    ///         Err(why) => {
    ///             // handle error
    ///         }
    ///     }
    /// }
    pub async fn artist(&self, id: impl Into<u64>) -> HttpResult<Artist, MusicError> {
        let builder = self.http.get(endpoint(format!("/lyrics/artist/{}/", id.into())).as_str());

        make_request::<Artist, MusicError>(builder).await
    }

    /// Get album information by a given ID
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.album(88151u64).await {
    ///     match res {
    ///         Ok(album) => {
    ///             // do something with the album
    ///         },
    ///         Err(why) => {
    ///             // handle error
    ///         }
    ///     }
    /// }
    pub async fn album(&self, id: impl Into<u64>) -> HttpResult<Album, MusicError> {
        let builder = self.http.get(endpoint(format!("/lyrics/album/{}/", id.into())).as_str());

        make_request::<Album, MusicError>(builder).await
    }

    /// Get album information by a given ID
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Ok(res) = client.music.track(2656006u64).await {
    ///     match res {
    ///         Ok(track) => {
    ///             // do something with the track
    ///         },
    ///         Err(why) => {
    ///             // handle error
    ///         }
    ///     }
    /// }
    pub async fn track(&self, id: impl Into<u64>) -> HttpResult<Track, MusicError> {
        let builder = self.http.get(endpoint(format!("/lyrics/track/{}/", id.into())).as_str());

        make_request::<Track, MusicError>(builder).await
    }
}
