use reqwest::{Client as HttpClient, Result as HttpResult};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::ApiResponse;
use crate::model::*;
use crate::{make_request, endpoint};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Music {
    pub http: Arc<HttpClient>
}

impl Music {
    pub fn new(http_client: Arc<HttpClient>) -> Self {
        Self {
            http: http_client
        }
    }

    pub async fn advanced_lyrics(&self, q: impl ToString, text_only: bool,
                                 limit: u32) -> HttpResult<ApiResponse<Lyrics, Error404>> {
        let builder = self.http.clone().get(endpoint("/lyrics/search").as_str())
            .query(&[("q", q.to_string())])
            .query(&[("text_only", text_only)])
            .query(&[("limit", limit)]);

        make_request::<Lyrics, Error404>(builder).await
    }

    pub async fn lyrics(&self, query: impl ToString) -> HttpResult<ApiResponse<Lyrics, Error404>> {
        self.advanced_lyrics(query.to_string(), false, 10).await
    }

    pub async fn advanced_recommendations(&self, tracks: ProviderType, youtube_token: Option<String>, limit: Option<u32>, recommend_type: Option<String>) -> HttpResult<MusicRecommendationsResponse>{
        let track_vec = match &tracks {
            ProviderType::Youtube(t) => t.clone(),
            ProviderType::YoutubeIDs(t) => t.clone(),
            ProviderType::YoutubeTitles(t) => t.clone(),
            ProviderType::SpotifyIDs(t) => t.clone()
        };
        let payload = MusicRecommendations {
            tracks: track_vec,
            provider: tracks.to_string(),
            youtube_token,
            limit,
            recommend_type
        };

        let response = self.http.clone().post(endpoint("/music/recommendations").as_str())
            .json(&payload)
            .send()
            .await?;

        response.json::<MusicRecommendationsResponse>().await
    }

    pub async fn recommendations(&self, tracks: ProviderType) -> HttpResult<MusicRecommendationsResponse> {
        self.advanced_recommendations(tracks, None, None, None).await
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Lyrics {
    pub total: u64,
    pub tool: u64,
    pub data: Vec<LyricsData>
}

#[derive(Clone, Debug, Deserialize)]
pub struct LyricsData {
    pub artist: String,
    pub artist_id: u64,
    pub album: String,
    pub album_ids: String,
    pub album_year: String,
    pub name: String,
    pub lyrics: String,
    pub search_str: String,
    pub album_art: String,
    pub popularity: u64,
    pub singalong: Vec<LyricsSingalong>,
    pub meta: LyricsMeta,
    pub id: String,
    pub search_score: f64,
    pub url: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct LyricsSingalong {
    pub lrc_timestamp: Option<String>,
    pub milliseconds: Option<String>,
    pub duration: Option<String>,
    pub line: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct LyricsMeta {
    pub spotify: SpotifyMeta,
    pub deezer: DeezerMeta,
    pub artists: ArtistsMeta,
    pub other: OtherMeta
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyMeta {
    pub artists: Vec<String>,
    pub track: String,
    pub album: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeezerMeta {
    pub artists: Vec<String>,
    pub track: String,
    pub album: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct ArtistsMeta {
    pub name: String,
    pub is_primary: bool,
    pub id: u64
}

#[derive(Clone, Debug, Deserialize)]
pub struct OtherMeta {
    pub gain: i64,
    pub bpm: f64
}

#[derive(Clone, Debug, Serialize)]
pub struct MusicRecommendations {
    pub tracks: Vec<String>,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommend_type: Option<String>,
}

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

#[derive(Clone, Debug, Deserialize)]
pub struct MusicRecommendationsResponse {
    pub provider: String,
    pub total: u32,
    pub tracks: Vec<RecommendationTrack>
}

#[derive(Clone, Debug, Deserialize)]
pub struct RecommendationTrack {
    pub youtube: YoutubeTrack,
    pub spotify: SpotifyTrack,
    pub name: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyTrack {
    pub id: String,
    pub album: SpotifyAlbum,
    pub artists: Vec<SpotifyArtist>,
    pub name: String,
    pub link: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyArtist {
    pub name: String,
    pub link: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyAlbum {
    pub name: String,
    pub album_art: String,
    pub link: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct YoutubeTrack {
    pub id: String,
    pub link: String,
    pub title: String,
    pub thumbnail: String,
    pub description: String,
}