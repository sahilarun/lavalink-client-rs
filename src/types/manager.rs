use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::types::queue::ManagerQueueOptions;
use crate::types::track::Track;

/// The Bot client Options needed for the manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotClientOptions {
    /// Bot Client Id
    pub id: String,
    /// Bot Client Username
    pub username: Option<String>,
    /// So users can pass entire objects / classes
    #[serde(flatten)]
    pub unknown_fields: HashMap<String, serde_json::Value>,
}

/// Sub Manager Options, for player specific things
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManagerPlayerOptions {
    /// If the Lavalink Volume should be decremented by x number
    pub volume_decrementer: Option<f64>,
    /// How often it should update the the player Position
    pub client_based_position_update_interval: Option<u64>,
    /// What should be used as a searchPlatform, if no source was provided during the query
    pub default_search_platform: Option<String>,
    /// Allow custom sources which lavalink-client does not support (yet)
    pub allow_custom_sources: Option<bool>,
    /// Applies the volume via a filter, not via the lavalink volume transformer
    pub apply_volume_as_filter: Option<bool>,
    /// What lavalink-client should do when the player reconnects
    pub on_disconnect: Option<PlayerOnDisconnectOptions>,
    /// Minimum time to play the song before autoPlayFunction is executed (prevents error spamming) Set to 0 to disable it @default 10000
    pub min_auto_play_ms: Option<u64>,
    /// Allows you to declare how many tracks are allowed to error/stuck within a time-frame before player is destroyed @default "{threshold: 35000, maxAmount: 3 }"
    pub max_errors_per_time: Option<MaxErrorsPerTimeOptions>,
    /* What the Player should do, when the queue gets empty */
    pub on_empty_queue: Option<OnEmptyQueueOptions>,
    /* If to override the data from the Unresolved Track. for unresolved tracks */
    pub use_unresolved_data: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerOnDisconnectOptions {
    /// Try to reconnect? -> If fails -> Destroy
    pub auto_reconnect: Option<bool>,
    /// Only try to reconnect if there are tracks in the queue
    pub auto_reconnect_only_with_tracks: Option<bool>,
    /// Instantly destroy player (overrides autoReconnect) | Don't provide == disable feature
    pub destroy_player: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxErrorsPerTimeOptions {
    /// The threshold time to count errors (recommended is 35s)
    pub threshold: u64,
    /// The max amount of errors within the threshold time which are allowed before destroying the player (when errors > maxAmount -> player.destroy())
    pub max_amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnEmptyQueueOptions {
    /* aut. destroy the player after x ms, if 1 it instantly destroys, don't provide or set to 0 to not destroy the player */
    pub destroy_after_ms: Option<u64>,
}

/// Manager Options used to create the manager
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManagerOptions {
    /// The Bot Client's Data for Authorization
    pub client: Option<BotClientOptions>,
    /// QueueOptions for all Queues
    #[serde(skip)]
    pub queue_options: Option<ManagerQueueOptions>,
    /// PlayerOptions for all Players
    pub player_options: Option<ManagerPlayerOptions>,
    /// If it should skip to the next Track on TrackEnd / TrackError etc. events
    pub auto_skip: Option<bool>,
    /// If it should automatically move the player to the next node when node is down
    pub auto_move: Option<bool>,
    /// If it should skip to the next Track if track.resolve errors while trying to play a track.
    pub auto_skip_on_resolve_error: Option<bool>,
    /// If it should emit only new (unique) songs and not when a looping track (or similar) is plaid, default false
    pub emit_new_songs_only: Option<bool>,
    /// Only allow link requests with links either matching some of that regExp or including some of that string
    pub links_whitelist: Option<Vec<String>>,
    /// Never allow link requests with links either matching some of that regExp or including some of that string (doesn't even allow if it's whitelisted)
    pub links_blacklist: Option<Vec<String>>,
    /// If links should be allowed or not. If set to false, it will throw an error if a link was provided.
    pub links_allowed: Option<bool>,
    /// Advanced Options for the Library, which may or may not be "library breaking"
    pub advanced_options: Option<AdvancedOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedOptions {
    /// Max duration for that the filter fix duration works (in ms) - default is 8mins
    pub max_filter_fix_duration: Option<u64>,
    /// Enable Debug event
    pub enable_debug_events: Option<bool>,
    /// optional
    pub debug_options: Option<DebugOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugOptions {
    /// For logging custom searches
    pub log_custom_searches: Option<bool>,
    /// logs for debugging the "no-Audio" playing error
    pub no_audio: Option<bool>,
    /// For Logging the Destroy function
    pub player_destroy: Option<PlayerDestroyDebugOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDestroyDebugOptions {
    /// To show the debug reason at all times.
    pub debug_log: Option<bool>,
    /// If you get 'Error: Use Player#destroy("reason") not LavalinkManager#deletePlayer() to stop the Player' put it on true
    pub dont_throw_error: Option<bool>,
}
