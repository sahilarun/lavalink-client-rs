use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum AudioOutputs {
    Mono,
    Stereo,
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFilters {
    pub custom: bool,
    pub nightcore: bool,
    pub vaporwave: bool,
    pub rotation: bool,
    pub karaoke: bool,
    pub tremolo: bool,
    pub vibrato: bool,
    pub low_pass: bool,
    pub audio_output: Option<AudioOutputs>,
    pub node_link_echo: bool,
    pub node_link_chorus: bool,
    pub node_link_compressor: bool,
    pub node_link_high_pass: bool,
    pub node_link_phaser: bool,
    pub node_link_spatial: bool,
    pub volume: bool,
    #[serde(default)]
    pub lavalink_filter_plugin: LavalinkFilterPluginState,
    #[serde(default)]
    pub lavalink_lava_dspx_plugin: LavalinkLavaDspxPluginState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkFilterPluginState {
    pub echo: bool,
    pub reverb: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkLavaDspxPluginState {
    pub low_pass: bool,
    pub high_pass: bool,
    pub normalization: bool,
    pub echo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EQBand {
    pub band: u8,
    pub gain: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KaraokeFilter {
    pub level: Option<f32>,
    pub mono_level: Option<f32>,
    pub filter_band: Option<f32>,
    pub filter_width: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimescaleFilter {
    pub speed: Option<f32>,
    pub pitch: Option<f32>,
    pub rate: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TremoloFilter {
    pub frequency: Option<f32>,
    pub depth: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VibratoFilter {
    pub frequency: Option<f32>,
    pub depth: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RotationFilter {
    pub rotation_hz: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DistortionFilter {
    pub sin_offset: Option<f32>,
    pub sin_scale: Option<f32>,
    pub cos_offset: Option<f32>,
    pub cos_scale: Option<f32>,
    pub tan_offset: Option<f32>,
    pub tan_scale: Option<f32>,
    pub offset: Option<f32>,
    pub scale: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChannelMixFilter {
    pub left_to_left: Option<f32>,
    pub left_to_right: Option<f32>,
    pub right_to_left: Option<f32>,
    pub right_to_right: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeLinkEchoFilter {
    pub delay: Option<f32>,
    pub feedback: Option<f32>,
    pub mix: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeLinkChorusFilter {
    pub rate: Option<f32>,
    pub depth: Option<f32>,
    pub delay: Option<f32>,
    pub mix: Option<f32>,
    pub feedback: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeLinkCompressorFilter {
    pub threshold: Option<f32>,
    pub ratio: Option<f32>,
    pub attack: Option<f32>,
    pub release: Option<f32>,
    pub gain: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkHighPassFilter {
    pub smoothing: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeLinkPhaserFilter {
    pub stages: Option<i32>,
    pub rate: Option<f32>,
    pub depth: Option<f32>,
    pub feedback: Option<f32>,
    pub mix: Option<f32>,
    pub min_frequency: Option<f32>,
    pub max_frequency: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeLinkSpatialFilter {
    pub depth: Option<f32>,
    pub rate: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LowPassFilter {
    pub smoothing: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilterDataPluginFilters {
    #[serde(rename = "lavalink-filter-plugin")]
    pub lavalink_filter_plugin: Option<PluginFilterLavalinkFilter>,
    #[serde(rename = "high-pass")]
    pub high_pass: Option<PluginFilterHighPass>,
    #[serde(rename = "low-pass")]
    pub low_pass: Option<PluginFilterLowPass>,
    pub normalization: Option<PluginFilterNormalization>,
    pub echo: Option<PluginFilterEcho>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginFilterLavalinkFilter {
    pub echo: Option<PluginFilterEchoBasic>,
    pub reverb: Option<PluginFilterReverb>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginFilterEchoBasic {
    pub delay: Option<f32>,
    pub decay: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginFilterReverb {
    pub delays: Option<Vec<f32>>,
    pub gains: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginFilterHighPass {
    pub cutoff_frequency: Option<f32>,
    pub boost_factor: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginFilterLowPass {
    pub cutoff_frequency: Option<f32>,
    pub boost_factor: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginFilterNormalization {
    pub max_amplitude: Option<f32>,
    pub adaptive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PluginFilterEcho {
    pub echo_length: Option<f32>,
    pub decay: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilterData {
    pub volume: Option<f32>,
    pub karaoke: Option<KaraokeFilter>,
    pub timescale: Option<TimescaleFilter>,
    pub tremolo: Option<TremoloFilter>,
    pub vibrato: Option<VibratoFilter>,
    pub rotation: Option<RotationFilter>,
    pub distortion: Option<DistortionFilter>,
    pub channel_mix: Option<ChannelMixFilter>,
    pub low_pass: Option<LowPassFilter>,
    pub echo: Option<NodeLinkEchoFilter>,
    pub chorus: Option<NodeLinkChorusFilter>,
    pub compressor: Option<NodeLinkCompressorFilter>,
    pub high_pass: Option<NodeLinkHighPassFilter>,
    pub phaser: Option<NodeLinkPhaserFilter>,
    pub spatial: Option<NodeLinkSpatialFilter>,
    pub plugin_filters: Option<FilterDataPluginFilters>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkFilterData {
    pub volume: Option<f32>,
    pub karaoke: Option<KaraokeFilter>,
    pub timescale: Option<TimescaleFilter>,
    pub tremolo: Option<TremoloFilter>,
    pub vibrato: Option<VibratoFilter>,
    pub rotation: Option<RotationFilter>,
    pub channel_mix: Option<ChannelMixFilter>,
    pub low_pass: Option<LowPassFilter>,
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
    // EqBand is passed down but kept separate to easily strip if empty 
    pub equalizer: Option<Vec<EQBand>>,
}

// In rust, FilterManager will likely be a struct tied to the `Player` state or passed to node updates directly.
// FilterManager logic is moved to `player.rs` where the actual REST updates are made, but for now we provide a stub struct.
#[derive(Debug, Clone)]
pub struct FilterManager {
    pub equalizer_bands: Vec<EQBand>,
    pub filter_updated_state: bool,
    pub filters: PlayerFilters,
    pub data: FilterData,
}

impl FilterManager {
    pub fn new() -> Self {
        Self {
            equalizer_bands: vec![],
            filter_updated_state: false,
            filters: PlayerFilters::default(),
            data: FilterData::default(),
        }
    }
}
