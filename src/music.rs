use reqwest::{Client as HttpClient, Result as HttpResult};
use std::sync::Arc;
use serde::Deserialize;
use crate::ApiResponse;
use crate::model::error::*;
use crate::make_request;

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
        let builder = self.http.clone().get("/lyrics/search")
            .query(&[("q", q.to_string())])
            .query(&[("text_only", text_only)])
            .query(&[("limit", limit)]);

        make_request::<Lyrics, Error404>(builder).await
    }

    pub async fn lyrics(&self, query: impl ToString) -> HttpResult<ApiResponse<Lyrics, Error404>> {
        self.advanced_lyrics(query.as_ref(), false, 10).await
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