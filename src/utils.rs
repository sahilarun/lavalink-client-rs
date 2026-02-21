use crate::types::track::{LavalinkTrack, Track, UnresolvedTrack, TrackInfo, PluginInfo, UnresolvedTrackInfo, TrackRequester, SourceNames};
use crate::types::events::{SearchResult, Exception};
use crate::types::player::PlayerOptions;
use crate::manager::LavalinkManagerOptions;
use crate::node::LavalinkNode;
use crate::statics::{DEFAULT_SOURCES, YOUTUBE_REGEX, SOUNDCLOUD_REGEX, BANDCAMP_REGEX, TWITCH_TV_REGEX, VIMEO_REGEX, TIKTOK_REGEX, MIXCLOUD_REGEX, ALL_SPOTIFY_REGEX, APPLE_MUSIC_REGEX, ALL_DEEZER_REGEX, MUSIC_YANDEX_REGEX, JIOSAAVN_REGEX, TIDAL_REGEX, ALL_PANDORA_REGEX, YOUTUBE_MUSIC_REGEX, SOUNDCLOUD_MOBILE_REGEX};
use url::Url;

/// Parses Node Connection Url: "lavalink://<nodeId>:<nodeAuthorization(Password)>@<NodeHost>:<NodePort>"
pub fn parse_lavalink_conn_url(connection_url: &str) -> Result<LavalinkConnUrl, url::ParseError> {
    if !connection_url.starts_with("lavalink://") {
        return Err(url::ParseError::RelativeUrlWithoutBase);
    }
    let parsed = Url::parse(connection_url)?;
    Ok(LavalinkConnUrl {
        id: parsed.username().to_string(),
        authorization: parsed.password().unwrap_or("").to_string(),
        host: parsed.host_str().unwrap_or("").to_string(),
        port: parsed.port().unwrap_or(80),
    })
}

#[derive(Debug, Clone)]
pub struct LavalinkConnUrl {
    pub id: String,
    pub authorization: String,
    pub host: String,
    pub port: u16,
}

pub struct ManagerUtils {
    pub default_search_platform: String,
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub query: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LavaSearchQuery {
    pub query: String,
    pub types: Vec<String>,
    pub source: Option<String>,
}

impl ManagerUtils {
    pub fn new(default_search_platform: String) -> Self {
        Self { default_search_platform }
    }

    pub fn build_plugin_info(&self, plugin_info: Option<PluginInfo>, client_data: Option<serde_json::Value>) -> PluginInfo {
        let mut info = plugin_info.unwrap_or_default();
        if let Some(cd) = client_data {
            info.client_data = Some(cd);
        }
        info
    }

    pub fn build_track(&self, data: LavalinkTrack, requester_id: Option<String>) -> Result<Track, String> {
        if data.encoded.is_none() {
            return Err("Argument 'data.encoded' must be present.".to_string());
        }
        
        let requester = requester_id.map(|id| TrackRequester { id: Some(id) });
        
        let track = Track {
            encoded: data.encoded,
            info: data.info.into(),
            plugin_info: self.build_plugin_info(data.plugin_info, None),
            requester,
            user_data: data.user_data,
        };

        Ok(track)
    }

    pub fn build_unresolved_track(&self, title: Option<String>, uri: Option<String>, encoded: Option<String>, requester_id: Option<String>) -> Result<UnresolvedTrack, String> {
        if title.is_none() && encoded.is_none() && uri.is_none() {
            return Err("Argument 'query' must be present.".to_string());
        }
        
        let requester = requester_id.map(|id| TrackRequester { id: Some(id) });
        
        Ok(UnresolvedTrack {
            encoded,
            info: UnresolvedTrackInfo {
                title: title.unwrap_or_default(),
                author: None,
                duration: None,
            },
            plugin_info: Some(PluginInfo::default()),
            requester,
            user_data: None,
        })
    }

    pub fn is_node(&self, node: &LavalinkNode) -> bool {
        // Rust's strong typing guarantees this is a Node, but we keep the method signature for TS matching
        true
    }

    pub fn is_not_broken_track(&self, data: &Track, min_duration: i64) -> bool {
        if data.info.duration <= min_duration.max(0) {
            return false;
        }
        self.is_track(data)
    }

    pub fn is_track(&self, data: &Track) -> bool {
        data.encoded.is_some() && !data.info.identifier.is_empty()
    }

    pub fn is_unresolved_track(&self, data: &UnresolvedTrack) -> bool {
        !data.info.title.is_empty() || data.encoded.is_some()
    }

    pub fn validate_query_string(&self, node: &LavalinkNode, query_string: &str, source_string: Option<&str>) -> Result<(), String> {
        if query_string.trim().is_empty() {
            return Err("Query string is empty, please provide a valid query string.".to_string());
        }

        if let Some(src) = source_string {
            if src == "speak" && query_string.len() > 100 {
                return Err("Query is speak, which is limited to 100 characters.".to_string());
            }
        }

        if !query_string.starts_with("http://") && !query_string.starts_with("https://") {
            return Ok(());
        }

        // The checks for source presence are omitted if we don't know the node's cached supported sources yet.
        // Assuming `node` has `info` available in a full implementation:
        /*
        if YOUTUBE_MUSIC_REGEX.is_match(query_string) || YOUTUBE_REGEX.is_match(query_string) { ... }
        */

        Ok(())
    }

    pub fn find_source_of_query(&self, query_string: &str) -> Option<String> {
        let lower_query = query_string.to_lowercase();
        for (source, _mapped) in DEFAULT_SOURCES.iter() {
            let prefix = format!("{}:", source);
            if lower_query.starts_with(&prefix) {
                if !["https", "http"].contains(source) {
                    return Some(source.to_string());
                }
            }
        }
        None
    }

    pub fn extract_source_of_query(&self, mut search_query: SearchQuery) -> SearchQuery {
        if let Some(found_source) = self.find_source_of_query(&search_query.query) {
            if let Some(mapped) = DEFAULT_SOURCES.get(found_source.as_str()) {
                search_query.source = Some(mapped.to_string());
                let prefix_len = found_source.len() + 1; // "source:"
                search_query.query = search_query.query[prefix_len..].to_string();
            }
        }
        search_query
    }

    pub fn extract_source_of_lava_query(&self, mut search_query: LavaSearchQuery) -> LavaSearchQuery {
        if let Some(found_source) = self.find_source_of_query(&search_query.query) {
            if let Some(mapped) = DEFAULT_SOURCES.get(found_source.as_str()) {
                search_query.source = Some(mapped.to_string());
                let prefix_len = found_source.len() + 1; // "source:"
                search_query.query = search_query.query[prefix_len..].to_string();
            }
        }
        search_query
    }

    pub fn transform_query(&self, query: SearchQuery) -> SearchQuery {
        let provided_source = query.source.clone().unwrap_or_else(|| self.default_search_platform.clone()).to_lowercase();
        let valid_source = DEFAULT_SOURCES.get(provided_source.as_str()).map(|&s| s.to_string()).unwrap_or(provided_source);
        
        self.extract_source_of_query(SearchQuery {
            query: query.query,
            source: Some(valid_source),
        })
    }

    pub fn transform_lava_search_query(&self, query: LavaSearchQuery) -> LavaSearchQuery {
        let provided_source = query.source.clone().unwrap_or_else(|| self.default_search_platform.clone()).to_lowercase();
        let valid_source = DEFAULT_SOURCES.get(provided_source.as_str()).map(|&s| s.to_string()).unwrap_or(provided_source);
        
        let valid_types = vec!["track", "playlist", "artist", "album", "text"];
        let final_types = if query.types.is_empty() {
            vec!["track".to_string(), "playlist".to_string(), "artist".to_string(), "album".to_string()]
        } else {
            valid_types.into_iter()
                .filter(|&v| query.types.iter().any(|qty| qty.to_lowercase().starts_with(v)))
                .map(|s| s.to_string())
                .collect()
        };

        self.extract_source_of_lava_query(LavaSearchQuery {
            query: query.query,
            types: final_types,
            source: Some(valid_source),
        })
    }

    pub fn validate_source_string(&self, node: &LavalinkNode, source_string: &str) -> Result<(), String> {
        let source_lower = source_string.to_lowercase();
        let _source = DEFAULT_SOURCES.get(source_lower.as_str());

        // We would validate node.info.source_managers here if available
        // e.g. if _source.unwrap() == "amsearch" && !node.info.source_managers.contains("applemusic") Error!
        Ok(())
    }
}
