use serde::{Deserialize, Serialize};

/// The Base 64 encoded String
pub type Base64 = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SourceNames {
    Youtube,
    YoutubeMusic,
    Soundcloud,
    Bandcamp,
    Twitch,
    Deezer,
    Spotify,
    AppleMusic,
    YandexMusic,
    FloweryTts,
    VkMusic,
    Tidal,
    Qobuz,
    Pandora,
    Jiosaavn,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkTrackInfo {
    /// The Identifier of the Track
    pub identifier: String,
    /// The Track Title / Name
    pub title: String,
    /// The Name of the Author
    pub author: String,
    /// The duration of the Track
    pub length: i64,
    /// The URL of the artwork if available
    pub artwork_url: Option<String>,
    /// The URL (aka Link) of the Track called URI
    pub uri: Option<String>,
    /// The Source name of the Track, e.g. soundcloud, youtube, spotify
    pub source_name: SourceNames,
    /// Whether the audio is seekable
    pub is_seekable: bool,
    /// Whether the audio is of a live stream
    pub is_stream: bool,
    /// If isrc code is available, it's provided
    pub isrc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackInfo {
    pub identifier: String,
    pub title: String,
    pub author: String,
    pub duration: i64,
    pub artwork_url: Option<String>,
    pub uri: Option<String>,
    pub source_name: SourceNames,
    pub is_seekable: bool,
    pub is_stream: bool,
    pub isrc: Option<String>,
}

impl From<LavalinkTrackInfo> for TrackInfo {
    fn from(val: LavalinkTrackInfo) -> Self {
        TrackInfo {
            identifier: val.identifier,
            title: val.title,
            author: val.author,
            duration: val.length,
            artwork_url: val.artwork_url,
            uri: val.uri,
            source_name: val.source_name,
            is_seekable: val.is_seekable,
            is_stream: val.is_stream,
            isrc: val.isrc,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginInfo {
    pub r#type: Option<String>,
    pub album_name: Option<String>,
    pub album_url: Option<String>,
    pub album_art_url: Option<String>,
    pub artist_url: Option<String>,
    pub artist_artwork_url: Option<String>,
    pub preview_url: Option<String>,
    pub is_preview: Option<bool>,
    pub total_tracks: Option<i32>,
    pub identifier: Option<String>,
    pub artwork_url: Option<String>,
    pub author: Option<String>,
    pub url: Option<String>,
    pub uri: Option<String>,
    pub client_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackRequester {
    // Arbitrary requester data mapping to TypeScript anyObject or specific trait later
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkTrack {
    pub encoded: Option<Base64>,
    pub info: LavalinkTrackInfo,
    pub plugin_info: Option<PluginInfo>,
    pub user_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub encoded: Option<Base64>,
    pub info: TrackInfo,
    #[serde(default)]
    pub plugin_info: PluginInfo,
    pub requester: Option<TrackRequester>,
    pub user_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnresolvedTrackInfo {
    pub title: String,
    // Add optional partial track info fields here if needed
    pub author: Option<String>,
    pub duration: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnresolvedTrack {
    pub encoded: Option<Base64>,
    pub info: UnresolvedTrackInfo,
    pub plugin_info: Option<PluginInfo>,
    pub user_data: Option<serde_json::Value>,
    pub requester: Option<TrackRequester>,
}
