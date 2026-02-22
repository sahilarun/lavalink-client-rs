use crate::types::filters::{FilterData, EQBand, LavalinkFilterData};
use super::{track::{Track, UnresolvedTrack}, queue::StoredQueue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RepeatMode {
    Queue,
    Track,
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPing {
    pub ws: i64,
    pub lavalink: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOptions {
    pub guild_id: String,
    pub voice_channel_id: String,
    pub text_channel_id: Option<String>,
    pub volume: Option<i32>,
    pub vc_region: Option<String>,
    pub self_deaf: Option<bool>,
    pub self_mute: Option<bool>,
    pub node: Option<String>,
    pub insta_update_filters_fix: Option<bool>,
    pub apply_volume_as_filter: Option<bool>,
    pub custom_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkPlayerVoiceOptions {
    pub endpoint: Option<String>,
    pub session_id: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackPlayOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkPlayOptions {
    pub track: Option<TrackPlayOptions>,
    pub position: Option<i64>,
    pub end_time: Option<i64>,
    pub paused: Option<bool>,
    pub volume: Option<i32>,
    pub filters: Option<LavalinkFilterData>,
    pub voice: Option<LavalinkPlayerVoiceOptions>,
}

#[derive(Debug, Clone, Default)]
pub struct PlayOptions {
    pub track: Option<TrackPlayOptions>,
    pub position: Option<i64>,
    pub end_time: Option<i64>,
    pub paused: Option<bool>,
    pub volume: Option<i32>,
    pub filters: Option<LavalinkFilterData>,
    pub voice: Option<LavalinkPlayerVoiceOptions>,
    pub no_replace: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct VoiceState {
    pub self_deaf: bool,
    pub self_mute: bool,
    pub server_deaf: bool,
    pub server_mute: bool,
    pub suppress: bool,
}

impl Default for VoiceState {
    fn default() -> Self {
        Self {
            self_deaf: false,
            self_mute: false,
            server_deaf: false,
            server_mute: false,
            suppress: false,
        }
    }
}
