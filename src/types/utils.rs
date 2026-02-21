use serde::{Deserialize, Serialize};
use crate::types::track::{Track, UnresolvedTrack, PluginInfo};

pub type SearchPlatform = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistInfo {
    pub name: String,
    pub title: String,
    pub author: Option<String>,
    pub thumbnail: Option<String>,
    pub uri: Option<String>,
    pub selected_track: Option<Track>,
    pub duration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub load_type: String,
    pub exception: Option<Exception>,
    pub plugin_info: PluginInfo,
    pub playlist: Option<PlaylistInfo>,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnresolvedSearchResult {
    pub load_type: String,
    pub exception: Option<Exception>,
    pub plugin_info: PluginInfo,
    pub playlist: Option<PlaylistInfo>,
    pub tracks: Vec<UnresolvedTrack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exception {
    pub severity: String,
    pub message: String,
    pub cause: String,
    pub cause_stack_trace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvalidLavalinkRestRequest {
    pub timestamp: i64,
    pub status: i32,
    pub error: String,
    pub message: Option<String>,
    pub trace: Option<serde_json::Value>,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkPlayerVoice {
    pub token: String,
    pub endpoint: String,
    pub session_id: String,
    pub connected: Option<bool>,
    pub ping: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailingAddress {
    pub failing_address: String,
    pub failing_timestamp: i64,
    pub failing_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlanner {
    #[serde(rename = "class")]
    pub class_name: Option<String>, // RoutePlannerTypes
    pub details: Option<RoutePlannerDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlannerDetails {
    pub ip_block: RoutePlannerIpBlock,
    pub failing_addresses: Vec<FailingAddress>,
    pub rotate_index: Option<String>,
    pub ip_index: Option<String>,
    pub current_address: Option<String>,
    pub current_address_index: Option<String>,
    pub block_index: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutePlannerIpBlock {
    #[serde(rename = "type")]
    pub ip_type: String,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub resuming: bool,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildShardPayload {
    pub op: i32,
    pub d: GuildShardPayloadData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildShardPayloadData {
    pub guild_id: String,
    pub channel_id: Option<String>,
    pub self_mute: bool,
    pub self_deaf: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavaSearchFilteredResponse {
    pub info: PlaylistInfo,
    pub plugin_info: PluginInfo,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavaSearchResponse {
    pub tracks: Vec<Track>,
    pub albums: Vec<LavaSearchFilteredResponse>,
    pub artists: Vec<LavaSearchFilteredResponse>,
    pub playlists: Vec<LavaSearchFilteredResponse>,
    pub texts: Vec<LavaSearchTextResult>,
    pub plugin_info: PluginInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavaSearchTextResult {
    pub text: String,
    pub plugin_info: PluginInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    pub query: String,
    pub source: Option<String>,
    // we omit extraQueryUrlParams to keep it simple or we can add it as hashmap
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavaSearchQuery {
    pub query: String,
    pub source: String,
    pub types: Option<Vec<String>>,
}
