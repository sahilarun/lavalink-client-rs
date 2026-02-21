use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::types::filters::{ChannelMixFilter, EQBand, AudioOutputs};
use crate::types::node_link::NodeLinkEventTypes;

/// Debug events for more detailed logging
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DebugEvents {
    SetSponsorBlock,
    DeleteSponsorBlock,
    TrackEndReplaced,
    AutoplayExecution,
    AutoplayNoSongsAdded,
    AutoplayThresholdSpamLimiter,
    TriggerQueueEmptyInterval,
    QueueEnded,
    TrackStartNewSongsOnly,
    TrackStartNoTrack,
    ResumingFetchingError,
    PlayerUpdateNoPlayer,
    PlayerUpdateFilterFixApply,
    PlayerUpdateSuccess,
    HeartBeatTriggered,
    NoSocketOnDestroy,
    SocketCleanupError,
    SocketTerminateHeartBeatTimeout,
    TryingConnectWhileConnected,
    LavaSearchNothingFound,
    SearchNothingFound,
    ValidatingBlacklistLinks,
    ValidatingWhitelistLinks,
    TrackErrorMaxTracksErroredPerTime,
    TrackStuckMaxTracksErroredPerTime,
    PlayerDestroyingSomewhereElse,
    PlayerCreateNodeNotFound,
    PlayerPlayQueueEmptyTimeoutClear,
    PlayerPlayWithTrackReplace,
    PlayerPlayUnresolvedTrack,
    PlayerPlayUnresolvedTrackFailed,
    PlayerVolumeAsFilter,
    BandcampSearchLokalEngine,
    PlayerChangeNode,
    BuildTrackError,
    TransformRequesterFunctionFailed,
    GetClosestTrackFailed,
    PlayerDeleteInsteadOfDestroy,
    FailedToConnectToNodes,
    NoAudioDebug,
    PlayerAutoReconnect,
    PlayerDestroyFail,
    PlayerChangeNodeFailNoEligibleNode,
    PlayerChangeNodeFail,
}

impl DebugEvents {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SetSponsorBlock => "SetSponsorBlock",
            Self::DeleteSponsorBlock => "DeleteSponsorBlock",
            Self::TrackEndReplaced => "TrackEndReplaced",
            Self::AutoplayExecution => "AutoplayExecution",
            Self::AutoplayNoSongsAdded => "AutoplayNoSongsAdded",
            Self::AutoplayThresholdSpamLimiter => "AutoplayThresholdSpamLimiter",
            Self::TriggerQueueEmptyInterval => "TriggerQueueEmptyInterval",
            Self::QueueEnded => "QueueEnded",
            Self::TrackStartNewSongsOnly => "TrackStartNewSongsOnly",
            Self::TrackStartNoTrack => "TrackStartNoTrack",
            Self::ResumingFetchingError => "ResumingFetchingError",
            Self::PlayerUpdateNoPlayer => "PlayerUpdateNoPlayer",
            Self::PlayerUpdateFilterFixApply => "PlayerUpdateFilterFixApply",
            Self::PlayerUpdateSuccess => "PlayerUpdateSuccess",
            Self::HeartBeatTriggered => "HeartBeatTriggered",
            Self::NoSocketOnDestroy => "NoSocketOnDestroy",
            Self::SocketCleanupError => "SocketCleanupError",
            Self::SocketTerminateHeartBeatTimeout => "SocketTerminateHeartBeatTimeout",
            Self::TryingConnectWhileConnected => "TryingConnectWhileConnected",
            Self::LavaSearchNothingFound => "LavaSearchNothingFound",
            Self::SearchNothingFound => "SearchNothingFound",
            Self::ValidatingBlacklistLinks => "ValidatingBlacklistLinks",
            Self::ValidatingWhitelistLinks => "ValidatingWhitelistLinks",
            Self::TrackErrorMaxTracksErroredPerTime => "TrackErrorMaxTracksErroredPerTime",
            Self::TrackStuckMaxTracksErroredPerTime => "TrackStuckMaxTracksErroredPerTime",
            Self::PlayerDestroyingSomewhereElse => "PlayerDestroyingSomewhereElse",
            Self::PlayerCreateNodeNotFound => "PlayerCreateNodeNotFound",
            Self::PlayerPlayQueueEmptyTimeoutClear => "PlayerPlayQueueEmptyTimeoutClear",
            Self::PlayerPlayWithTrackReplace => "PlayerPlayWithTrackReplace",
            Self::PlayerPlayUnresolvedTrack => "PlayerPlayUnresolvedTrack",
            Self::PlayerPlayUnresolvedTrackFailed => "PlayerPlayUnresolvedTrackFailed",
            Self::PlayerVolumeAsFilter => "PlayerVolumeAsFilter",
            Self::BandcampSearchLokalEngine => "BandcampSearchLokalEngine",
            Self::PlayerChangeNode => "PlayerChangeNode",
            Self::BuildTrackError => "BuildTrackError",
            Self::TransformRequesterFunctionFailed => "TransformRequesterFunctionFailed",
            Self::GetClosestTrackFailed => "GetClosestTrackFailed",
            Self::PlayerDeleteInsteadOfDestroy => "PlayerDeleteInsteadOfDestroy",
            Self::FailedToConnectToNodes => "FailedToConnectToNodes",
            Self::NoAudioDebug => "NoAudioDebug",
            Self::PlayerAutoReconnect => "PlayerAutoReconnect",
            Self::PlayerDestroyFail => "PlayerDestroyFail",
            Self::PlayerChangeNodeFailNoEligibleNode => "PlayerChangeNodeFailNoEligibleNode",
            Self::PlayerChangeNodeFail => "PlayerChangeNodeFail",
        }
    }
}

/// Reasons why a player got destroyed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DestroyReasons {
    QueueEmpty,
    NodeDestroy,
    NodeDeleted,
    LavalinkNoVoice,
    NodeReconnectFail,
    Disconnected,
    PlayerReconnectFail,
    PlayerChangeNodeFail,
    PlayerChangeNodeFailNoEligibleNode,
    ChannelDeleted,
    DisconnectAllNodes,
    ReconnectAllNodes,
    TrackErrorMaxTracksErroredPerTime,
    TrackStuckMaxTracksErroredPerTime,
}

impl DestroyReasons {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueueEmpty => "QueueEmpty",
            Self::NodeDestroy => "NodeDestroy",
            Self::NodeDeleted => "NodeDeleted",
            Self::LavalinkNoVoice => "LavalinkNoVoice",
            Self::NodeReconnectFail => "NodeReconnectFail",
            Self::Disconnected => "Disconnected",
            Self::PlayerReconnectFail => "PlayerReconnectFail",
            Self::PlayerChangeNodeFail => "PlayerChangeNodeFail",
            Self::PlayerChangeNodeFailNoEligibleNode => "PlayerChangeNodeFailNoEligibleNode",
            Self::ChannelDeleted => "ChannelDeleted",
            Self::DisconnectAllNodes => "DisconnectAllNodes",
            Self::ReconnectAllNodes => "ReconnectAllNodes",
            Self::TrackErrorMaxTracksErroredPerTime => "TrackErrorMaxTracksErroredPerTime",
            Self::TrackStuckMaxTracksErroredPerTime => "TrackStuckMaxTracksErroredPerTime",
        }
    }
}

/// Reasons why a player got disconnected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisconnectReasons {
    Disconnected,
    DisconnectAllNodes,
}

impl DisconnectReasons {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::DisconnectAllNodes => "DisconnectAllNodes",
        }
    }
}

/// The valid SponsorBlock categories
pub const VALID_SPONSOR_BLOCKS: &[&str] = &[
    "sponsor",
    "selfpromo",
    "interaction",
    "intro",
    "outro",
    "preview",
    "music_offtopic",
    "filler",
];

lazy_static! {
    /// The audio Outputs Data map declaration
    pub static ref AUDIO_OUTPUTS_DATA: HashMap<AudioOutputs, ChannelMixFilter> = {
        let mut m = HashMap::new();
        m.insert(AudioOutputs::Mono, ChannelMixFilter {
            left_to_left: Some(0.5),
            left_to_right: Some(0.5),
            right_to_left: Some(0.5),
            right_to_right: Some(0.5),
        });
        m.insert(AudioOutputs::Stereo, ChannelMixFilter {
            left_to_left: Some(1.0),
            left_to_right: Some(0.0),
            right_to_left: Some(0.0),
            right_to_right: Some(1.0),
        });
        m.insert(AudioOutputs::Left, ChannelMixFilter {
            left_to_left: Some(1.0),
            left_to_right: Some(0.0),
            right_to_left: Some(1.0),
            right_to_right: Some(0.0),
        });
        m.insert(AudioOutputs::Right, ChannelMixFilter {
            left_to_left: Some(0.0),
            left_to_right: Some(1.0),
            right_to_left: Some(0.0),
            right_to_right: Some(1.0),
        });
        m
    };

    /// Equalizer Presets
    pub static ref EQ_LIST: HashMap<&'static str, Vec<EQBand>> = {
        let mut m = HashMap::new();
        
        m.insert("BassboostEarrape", vec![
            EQBand { band: 0, gain: 0.6 * 0.375 },
            EQBand { band: 1, gain: 0.67 * 0.375 },
            EQBand { band: 2, gain: 0.67 * 0.375 },
            EQBand { band: 3, gain: 0.4 * 0.375 },
            EQBand { band: 4, gain: -0.5 * 0.375 },
            EQBand { band: 5, gain: 0.15 * 0.375 },
            EQBand { band: 6, gain: -0.45 * 0.375 },
            EQBand { band: 7, gain: 0.23 * 0.375 },
            EQBand { band: 8, gain: 0.35 * 0.375 },
            EQBand { band: 9, gain: 0.45 * 0.375 },
            EQBand { band: 10, gain: 0.55 * 0.375 },
            EQBand { band: 11, gain: -0.6 * 0.375 },
            EQBand { band: 12, gain: 0.55 * 0.375 },
            EQBand { band: 13, gain: -0.5 * 0.375 },
            EQBand { band: 14, gain: -0.75 * 0.375 },
        ]);
        
        m.insert("BassboostHigh", vec![
            EQBand { band: 0, gain: 0.6 * 0.25 },
            EQBand { band: 1, gain: 0.67 * 0.25 },
            EQBand { band: 2, gain: 0.67 * 0.25 },
            EQBand { band: 3, gain: 0.4 * 0.25 },
            EQBand { band: 4, gain: -0.5 * 0.25 },
            EQBand { band: 5, gain: 0.15 * 0.25 },
            EQBand { band: 6, gain: -0.45 * 0.25 },
            EQBand { band: 7, gain: 0.23 * 0.25 },
            EQBand { band: 8, gain: 0.35 * 0.25 },
            EQBand { band: 9, gain: 0.45 * 0.25 },
            EQBand { band: 10, gain: 0.55 * 0.25 },
            EQBand { band: 11, gain: -0.6 * 0.25 },
            EQBand { band: 12, gain: 0.55 * 0.25 },
            EQBand { band: 13, gain: -0.5 * 0.25 },
            EQBand { band: 14, gain: -0.75 * 0.25 },
        ]);

        m.insert("BassboostMedium", vec![
            EQBand { band: 0, gain: 0.6 * 0.1875 },
            EQBand { band: 1, gain: 0.67 * 0.1875 },
            EQBand { band: 2, gain: 0.67 * 0.1875 },
            EQBand { band: 3, gain: 0.4 * 0.1875 },
            EQBand { band: 4, gain: -0.5 * 0.1875 },
            EQBand { band: 5, gain: 0.15 * 0.1875 },
            EQBand { band: 6, gain: -0.45 * 0.1875 },
            EQBand { band: 7, gain: 0.23 * 0.1875 },
            EQBand { band: 8, gain: 0.35 * 0.1875 },
            EQBand { band: 9, gain: 0.45 * 0.1875 },
            EQBand { band: 10, gain: 0.55 * 0.1875 },
            EQBand { band: 11, gain: -0.6 * 0.1875 },
            EQBand { band: 12, gain: 0.55 * 0.1875 },
            EQBand { band: 13, gain: -0.5 * 0.1875 },
            EQBand { band: 14, gain: -0.75 * 0.1875 },
        ]);

        m.insert("BassboostLow", vec![
            EQBand { band: 0, gain: 0.6 * 0.125 },
            EQBand { band: 1, gain: 0.67 * 0.125 },
            EQBand { band: 2, gain: 0.67 * 0.125 },
            EQBand { band: 3, gain: 0.4 * 0.125 },
            EQBand { band: 4, gain: -0.5 * 0.125 },
            EQBand { band: 5, gain: 0.15 * 0.125 },
            EQBand { band: 6, gain: -0.45 * 0.125 },
            EQBand { band: 7, gain: 0.23 * 0.125 },
            EQBand { band: 8, gain: 0.35 * 0.125 },
            EQBand { band: 9, gain: 0.45 * 0.125 },
            EQBand { band: 10, gain: 0.55 * 0.125 },
            EQBand { band: 11, gain: -0.6 * 0.125 },
            EQBand { band: 12, gain: 0.55 * 0.125 },
            EQBand { band: 13, gain: -0.5 * 0.125 },
            EQBand { band: 14, gain: -0.75 * 0.125 },
        ]);

        m.insert("BetterMusic", vec![
            EQBand { band: 0, gain: 0.25 },
            EQBand { band: 1, gain: 0.025 },
            EQBand { band: 2, gain: 0.0125 },
            EQBand { band: 3, gain: 0.0 },
            EQBand { band: 4, gain: 0.0 },
            EQBand { band: 5, gain: -0.0125 },
            EQBand { band: 6, gain: -0.025 },
            EQBand { band: 7, gain: -0.0175 },
            EQBand { band: 8, gain: 0.0 },
            EQBand { band: 9, gain: 0.0 },
            EQBand { band: 10, gain: 0.0125 },
            EQBand { band: 11, gain: 0.025 },
            EQBand { band: 12, gain: 0.25 },
            EQBand { band: 13, gain: 0.125 },
            EQBand { band: 14, gain: 0.125 },
        ]);

        m.insert("Rock", vec![
            EQBand { band: 0, gain: 0.3 },
            EQBand { band: 1, gain: 0.25 },
            EQBand { band: 2, gain: 0.2 },
            EQBand { band: 3, gain: 0.1 },
            EQBand { band: 4, gain: 0.05 },
            EQBand { band: 5, gain: -0.05 },
            EQBand { band: 6, gain: -0.15 },
            EQBand { band: 7, gain: -0.2 },
            EQBand { band: 8, gain: -0.1 },
            EQBand { band: 9, gain: -0.05 },
            EQBand { band: 10, gain: 0.05 },
            EQBand { band: 11, gain: 0.1 },
            EQBand { band: 12, gain: 0.2 },
            EQBand { band: 13, gain: 0.25 },
            EQBand { band: 14, gain: 0.3 },
        ]);

        m.insert("Classic", vec![
            EQBand { band: 0, gain: 0.375 },
            EQBand { band: 1, gain: 0.35 },
            EQBand { band: 2, gain: 0.125 },
            EQBand { band: 3, gain: 0.0 },
            EQBand { band: 4, gain: 0.0 },
            EQBand { band: 5, gain: 0.125 },
            EQBand { band: 6, gain: 0.55 },
            EQBand { band: 7, gain: 0.05 },
            EQBand { band: 8, gain: 0.125 },
            EQBand { band: 9, gain: 0.25 },
            EQBand { band: 10, gain: 0.2 },
            EQBand { band: 11, gain: 0.25 },
            EQBand { band: 12, gain: 0.3 },
            EQBand { band: 13, gain: 0.25 },
            EQBand { band: 14, gain: 0.3 },
        ]);

        m.insert("Pop", vec![
            EQBand { band: 0, gain: 0.2635 },
            EQBand { band: 1, gain: 0.22141 },
            EQBand { band: 2, gain: -0.21141 },
            EQBand { band: 3, gain: -0.1851 },
            EQBand { band: 4, gain: -0.155 },
            EQBand { band: 5, gain: 0.21141 },
            EQBand { band: 6, gain: 0.22456 },
            EQBand { band: 7, gain: 0.237 },
            EQBand { band: 8, gain: 0.237 },
            EQBand { band: 9, gain: 0.237 },
            EQBand { band: 10, gain: -0.05 },
            EQBand { band: 11, gain: -0.116 },
            EQBand { band: 12, gain: 0.192 },
            EQBand { band: 13, gain: 0.0 },
        ]);

        m.insert("Electronic", vec![
            EQBand { band: 0, gain: 0.375 },
            EQBand { band: 1, gain: 0.35 },
            EQBand { band: 2, gain: 0.125 },
            EQBand { band: 3, gain: 0.0 },
            EQBand { band: 4, gain: 0.0 },
            EQBand { band: 5, gain: -0.125 },
            EQBand { band: 6, gain: -0.125 },
            EQBand { band: 7, gain: 0.0 },
            EQBand { band: 8, gain: 0.25 },
            EQBand { band: 9, gain: 0.125 },
            EQBand { band: 10, gain: 0.15 },
            EQBand { band: 11, gain: 0.2 },
            EQBand { band: 12, gain: 0.25 },
            EQBand { band: 13, gain: 0.35 },
            EQBand { band: 14, gain: 0.4 },
        ]);

        m.insert("FullSound", vec![
            EQBand { band: 0, gain: 0.25 + 0.375 },
            EQBand { band: 1, gain: 0.25 + 0.025 },
            EQBand { band: 2, gain: 0.25 + 0.0125 },
            EQBand { band: 3, gain: 0.25 + 0.0 },
            EQBand { band: 4, gain: 0.25 + 0.0 },
            EQBand { band: 5, gain: 0.25 + -0.0125 },
            EQBand { band: 6, gain: 0.25 + -0.025 },
            EQBand { band: 7, gain: 0.25 + -0.0175 },
            EQBand { band: 8, gain: 0.25 + 0.0 },
            EQBand { band: 9, gain: 0.25 + 0.0 },
            EQBand { band: 10, gain: 0.25 + 0.0125 },
            EQBand { band: 11, gain: 0.25 + 0.025 },
            EQBand { band: 12, gain: 0.25 + 0.375 },
            EQBand { band: 13, gain: 0.25 + 0.125 },
            EQBand { band: 14, gain: 0.25 + 0.125 },
        ]);

        m.insert("Gaming", vec![
            EQBand { band: 0, gain: 0.35 },
            EQBand { band: 1, gain: 0.3 },
            EQBand { band: 2, gain: 0.25 },
            EQBand { band: 3, gain: 0.2 },
            EQBand { band: 4, gain: 0.15 },
            EQBand { band: 5, gain: 0.1 },
            EQBand { band: 6, gain: 0.05 },
            EQBand { band: 7, gain: -0.0 },
            EQBand { band: 8, gain: -0.05 },
            EQBand { band: 9, gain: -0.1 },
            EQBand { band: 10, gain: -0.15 },
            EQBand { band: 11, gain: -0.2 },
            EQBand { band: 12, gain: -0.25 },
            EQBand { band: 13, gain: -0.3 },
            EQBand { band: 14, gain: -0.35 },
        ]);

        m
    };
}

pub struct RecommendationsStrings;

impl RecommendationsStrings {
    pub fn high_cpu_load(cpu_load: f64) -> String {
        format!("High CPU load ({:.1}%). Consider reducing player count or upgrading CPU.", cpu_load * 100.0)
    }

    pub fn high_system_load(system_load: f64) -> String {
        format!("High system load ({:.1}%). Check other processes on the server.", system_load * 100.0)
    }

    pub fn high_memory_usage(memory_usage_percent: f64) -> String {
        format!("High memory usage ({:.1}%). Consider increasing allocated memory or reducing player count.", memory_usage_percent)
    }

    pub fn frame_deficit(frame_deficit: i64) -> String {
        format!("Frame deficit detected ({}). Audio quality may be affected. Check network and CPU.", frame_deficit)
    }

    pub fn high_latency(ping: i64) -> String {
        format!("High latency ({}ms). Check network connection to the node.", ping)
    }

    pub fn node_restart() -> &'static str {
        "Node restart recommended to clear memory and reset connections."
    }

    pub fn high_playercount(players: i64) -> String {
        format!("High player count ({}). Consider load balancing across multiple nodes.", players)
    }

    pub fn node_offline() -> &'static str {
        "Node is offline or disconnected"
    }

    pub fn check_connectivity() -> &'static str {
        "Check node connectivity and restart if needed"
    }
}

pub const NODE_LINK_EXCLUSIVE_EVENTS: &[NodeLinkEventTypes] = &[
    NodeLinkEventTypes::PlayerCreatedEvent,
    NodeLinkEventTypes::PlayerDestroyedEvent,
    NodeLinkEventTypes::PlayerConnectedEvent,
    NodeLinkEventTypes::PlayerReconnectingEvent,
    NodeLinkEventTypes::VolumeChangedEvent,
    NodeLinkEventTypes::FiltersChangedEvent,
    NodeLinkEventTypes::SeekEvent,
    NodeLinkEventTypes::PauseEvent,
    NodeLinkEventTypes::ConnectionStatusEvent,
    NodeLinkEventTypes::MixStartedEvent,
    NodeLinkEventTypes::MixEndedEvent,
    NodeLinkEventTypes::LyricsFoundEvent,
    NodeLinkEventTypes::LyricsLineEvent,
    NodeLinkEventTypes::LyricsNotFoundEvent,
];
