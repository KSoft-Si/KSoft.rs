use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct Lyrics {
    pub total: u64,
    pub took: u64,
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
    pub artists: Option<Vec<ArtistsMeta>>,
    pub other: OtherMeta
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpotifyMeta {
    pub artists: Vec<String>,
    pub track: Option<String>,
    pub album: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeezerMeta {
    pub artists: Vec<String>,
    pub track: Option<String>,
    pub album: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct ArtistsMeta {
    pub name: String,
    pub is_primary: bool,
    pub id: u64
}

#[derive(Clone, Debug, Deserialize)]
pub struct OtherMeta {
    pub gain: f64,
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

#[derive(Clone, Debug, Deserialize)]
pub struct Artist {
    pub id: u64,
    pub name: String,
    pub albums: Vec<ArtistAlbum>,
    pub tracks: Vec<ArtistTrack>
}

#[derive(Clone, Debug, Deserialize)]
pub struct ArtistAlbum {
    pub id: u64,
    pub name: String,
    pub year: u16
}

#[derive(Clone, Debug, Deserialize)]
pub struct ArtistTrack {
    pub id: u64,
    pub name: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Album {
    pub id: u64,
    pub name: String,
    pub year: u16,
    pub artist: AlbumArtist,
    pub tracks: Vec<ArtistTrack>
}

#[derive(Clone, Debug, Deserialize)]
pub struct AlbumArtist {
    pub id: u64,
    pub name: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Track {
    pub name: String,
    pub artist: AlbumArtist,
    pub albums: Vec<TrackAlbum>,
    pub lyrics: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct TrackAlbum {
    pub id: u64,
    pub name: String,
    pub year: u16
}