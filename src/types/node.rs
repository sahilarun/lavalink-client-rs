use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::types::track::PluginInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkNodeOptions {
    pub node_type: Option<String>,
    pub host: String,
    pub port: u16,
    pub authorization: String,
    pub secure: Option<bool>,
    pub session_id: Option<String>,
    pub id: Option<String>,
    pub regions: Option<Vec<String>>,
    pub retry_amount: Option<i32>,
    pub retry_delay: Option<i32>,
    pub retry_timespan: Option<i32>,
    pub request_signal_timeout_ms: Option<i32>,
    pub close_on_error: Option<bool>,
    pub heart_beat_interval: Option<i32>,
    pub enable_ping_on_stats_check: Option<bool>,
    pub auto_checks: Option<LavalinkNodeAutoChecks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkNodeAutoChecks {
    pub plugin_validations: Option<bool>,
    pub sources_validations: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub free: u64,
    pub used: u64,
    pub allocated: u64,
    pub reservable: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CPUStats {
    pub cores: i32,
    pub system_load: f64,
    pub lavalink_load: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameStats {
    pub sent: Option<i64>,
    pub nulled: Option<i64>,
    pub deficit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseNodeStats {
    pub players: i32,
    pub playing_players: i32,
    pub uptime: i64,
    pub memory: MemoryStats,
    pub cpu: CPUStats,
    pub frame_stats: FrameStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkConnectionMetrics {
    pub status: String,
    pub metrics: NodeLinkConnectionMetricsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkConnectionMetricsData {
    pub speed: NodeLinkConnectionMetricsSpeed,
    pub downloaded_bytes: i64,
    pub duration_seconds: i64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkConnectionMetricsSpeed {
    pub bps: f64,
    pub kbps: f64,
    pub mbps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStats {
    #[serde(flatten)]
    pub base: BaseNodeStats,
    pub detailed_stats: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkInfo {
    pub version: VersionObject,
    pub build_time: i64,
    pub git: GitObject,
    pub jvm: String,
    pub lavaplayer: String,
    pub source_managers: Vec<String>,
    pub filters: Vec<String>,
    pub plugins: Vec<PluginObject>,
    pub is_nodelink: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionObject {
    pub semver: String,
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub pre_release: Option<String>,
    pub build: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitObject {
    pub branch: String,
    pub commit: String,
    pub commit_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginObject {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsResult {
    pub source_name: String,
    pub provider: String,
    pub text: Option<String>,
    pub lines: Vec<LyricsLine>,
    pub plugin: PluginInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LyricsLine {
    pub timestamp: i64,
    pub duration: Option<i64>,
    pub line: String,
    pub plugin: PluginInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReconnectionState {
    Idle,
    Reconnecting,
    Pending,
    Destroying,
}
