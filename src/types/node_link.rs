use serde::{Deserialize, Serialize};
use crate::types::filters::LavalinkFilterData;
use crate::types::track::LavalinkTrack;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum NodeLinkEventTypes {
    PlayerCreatedEvent,
    PlayerDestroyedEvent,
    PlayerConnectedEvent,
    PlayerReconnectingEvent,
    VolumeChangedEvent,
    FiltersChangedEvent,
    SeekEvent,
    PauseEvent,
    ConnectionStatusEvent,
    MixStartedEvent,
    MixEndedEvent,
    LyricsFoundEvent,
    LyricsLineEvent,
    LyricsNotFoundEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkBaseEvent {
    pub op: String, // "event"
    #[serde(rename = "type")]
    pub event_type: String,
    pub guild_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeChangedEvent {
    #[serde(flatten)]
    pub base: NodeLinkBaseEvent,
    pub volume: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiltersChangedEvent {
    #[serde(flatten)]
    pub base: NodeLinkBaseEvent,
    pub filters: LavalinkFilterData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeekEvent {
    #[serde(flatten)]
    pub base: NodeLinkBaseEvent,
    pub position: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PauseEvent {
    #[serde(flatten)]
    pub base: NodeLinkBaseEvent,
    pub paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusEvent {
    #[serde(flatten)]
    pub base: NodeLinkBaseEvent,
    pub connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixStartedEvent {
    #[serde(flatten)]
    pub base: NodeLinkBaseEvent,
    pub mix_id: String,
    pub track: LavalinkTrack,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixEndedEvent {
    #[serde(flatten)]
    pub base: NodeLinkBaseEvent,
    pub mix_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatusThreshold {
    pub excellent: f64,
    pub good: f64,
    pub fair: f64,
    pub poor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthStatusThresholdOptions {
    pub cpu: Option<HealthStatusThreshold>,
    pub memory: Option<HealthStatusThreshold>,
    pub ping: Option<HealthStatusThreshold>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeMetricSummary {
    pub cpu_load: f64,
    pub system_load: f64,
    pub memory_usage: f64,
    pub players: i32,
    pub playing_players: i32,
    pub uptime: i64,
    pub ping: i64,
    pub frame_deficit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthStatusObject {
    pub status: String, // HealthStatusKeys
    pub performance: String, // HealthPerformanceKeys
    pub is_overloaded: bool,
    pub needs_restart: bool,
    pub penalty_score: f64,
    pub estimated_remaining_capacity: f64,
    pub recommendations: Vec<String>,
    pub metrics: NodeMetricSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMixerLayerResponse {
    pub id: String,
    pub track: LavalinkTrack,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixDetails {
    pub id: String,
    pub track: LavalinkTrack,
    pub volume: f64,
    pub position: i64,
    pub start_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMixerLayersResponse {
    pub mixes: Vec<MixDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionMetricsResponse {
    pub status: String,
    pub metrics: ConnectionMetricsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionMetricsData {
    pub speed: ConnectionMetricsSpeed,
    pub downloaded_bytes: i64,
    pub duration_seconds: i64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionMetricsSpeed {
    pub bps: f64,
    pub kbps: f64,
    pub mbps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkLyricsSynced {
    pub load_type: String,
    pub data: NodeLinkLyricsSyncedData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkLyricsSyncedData {
    pub synced: bool, // true
    pub lang: String,
    pub source: String,
    pub lines: Vec<NodeLinkLyricsLineSynced>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkLyricsLineSynced {
    pub text: String,
    pub time: i64,
    pub duration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkLyricsPlain {
    pub load_type: String,
    pub data: NodeLinkLyricsPlainData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkLyricsPlainData {
    pub synced: bool, // false
    pub lang: String,
    pub source: String,
    pub lines: Vec<NodeLinkLyricsLinePlain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkLyricsLinePlain {
    pub text: String,
    pub time: Option<i64>,
    pub duration: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeLinkLyrics {
    Synced(NodeLinkLyricsSynced),
    Plain(NodeLinkLyricsPlain),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkNoLyrics {
    pub load_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkChapter {
    pub title: String,
    pub start_time: i64,
    pub thumbnails: Vec<NodeLinkThumbnail>,
    pub duration: i64,
    pub end_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkThumbnail {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectStreamResponse {
    pub url: String,
    pub protocol: String,
    pub format: String,
    pub hls_url: Option<String>,
    pub formats: Vec<DirectStreamFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectStreamFormat {
    pub itag: i32,
    pub mime_type: String,
    pub quality_label: String,
    pub bitrate: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YoutubeOAuthResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub scope: String,
    pub token_type: String,
}
