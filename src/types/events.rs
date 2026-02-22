use serde::{Deserialize, Serialize};
use crate::types::track::{LavalinkTrack, PluginInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exception {
    pub severity: String,
    pub message: Option<String>,
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PlayerEvent {
    TrackStartEvent {
        #[serde(rename = "guildId")]
        guild_id: String,
        track: LavalinkTrack,
    },
    TrackEndEvent {
        #[serde(rename = "guildId")]
        guild_id: String,
        track: LavalinkTrack,
        reason: String,
    },
    TrackExceptionEvent {
        #[serde(rename = "guildId")]
        guild_id: String,
        exception: Option<Exception>,
        track: LavalinkTrack,
        error: String,
    },
    TrackStuckEvent {
        #[serde(rename = "guildId")]
        guild_id: String,
        #[serde(rename = "thresholdMs")]
        threshold_ms: i64,
        track: LavalinkTrack,
    },
    WebSocketClosedEvent {
        #[serde(rename = "guildId")]
        guild_id: String,
        code: i32,
        #[serde(rename = "byRemote")]
        by_remote: bool,
        reason: String,
    },
    // Add SponsorBlock and Lyrics events if necessary, basic support here
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkPlayerVoice {
    pub token: Option<String>,
    pub endpoint: Option<String>,
    pub session_id: Option<String>,
    pub connected: Option<bool>,
    pub ping: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkPlayerState {
    pub time: i64,
    pub position: i64,
    pub connected: bool,
    pub ping: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkPlayer {
    pub guild_id: String,
    pub track: Option<LavalinkTrack>,
    pub volume: i32,
    pub paused: bool,
    pub voice: LavalinkPlayerVoice,
    pub state: LavalinkPlayerState,
    #[serde(default)]
    pub filters: crate::types::filters::FilterData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum LavalinkMessage {
    #[serde(rename = "ready")]
    Ready {
        resumed: bool,
        #[serde(rename = "sessionId")]
        session_id: String,
    },
    #[serde(rename = "stats")]
    Stats(crate::node::NodeStats),
    #[serde(rename = "playerUpdate")]
    PlayerUpdate {
        #[serde(rename = "guildId")]
        guild_id: String,
        state: LavalinkPlayerState,
    },
    #[serde(rename = "event")]
    Event(PlayerEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistInfoData {
    pub name: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub thumbnail: Option<String>,
    pub uri: Option<String>,
    pub selected_track: Option<i64>,
    pub duration: Option<i64>,
    pub tracks: Option<Vec<LavalinkTrack>>,
}

/// Lavalink v4 loadtracks response.
/// `data` is polymorphic:
///   - `loadType = "search"` or `"track"` → data is a JSON array of tracks
///   - `loadType = "playlist"`             → data is a JSON object (PlaylistInfoData)
///   - `loadType = "error"`               → data is a JSON object (Exception-like)
///   - `loadType = "empty"`               → data is absent / null
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub load_type: String,
    #[serde(default)]
    pub data: serde_json::Value,
    // Legacy flat fields (kept for forwards-compat, may be None for Ibuki)
    #[serde(default)]
    pub plugin_info: Option<PluginInfo>,
}

impl SearchResult {
    /// Returns the list of tracks when loadType is "search" or "track".
    pub fn tracks(&self) -> Vec<LavalinkTrack> {
        match &self.data {
            serde_json::Value::Array(arr) => arr
                .iter()
                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                .collect(),
            serde_json::Value::Object(map) => {
                if let Some(t) = map.get("tracks") {
                    if let serde_json::Value::Array(arr) = t {
                        return arr
                            .iter()
                            .filter_map(|v| serde_json::from_value(v.clone()).ok())
                            .collect();
                    }
                }
                vec![]
            }
            _ => vec![],
        }
    }

    /// Returns playlist info when loadType is "playlist".
    pub fn playlist(&self) -> Option<PlaylistInfoData> {
        if self.load_type == "playlist" {
            serde_json::from_value(self.data.clone()).ok()
        } else {
            None
        }
    }

    /// Returns the error Exception when loadType is "error".
    pub fn error(&self) -> Option<Exception> {
        if self.load_type == "error" {
            serde_json::from_value(self.data.clone()).ok()
        } else {
            None
        }
    }
}

