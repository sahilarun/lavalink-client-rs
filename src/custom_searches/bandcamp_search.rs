use crate::player::Player;
use crate::types::track::{UnresolvedTrackInfo, TrackRequester};
use crate::types::utils::UnresolvedSearchResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandCampAutocompleteTrackObject {
    pub url: Option<String>,
    pub uri: Option<String>,
    pub img: Option<String>,
    pub band_name: Option<String>,
    pub name: Option<String>,
    pub id: Option<u64>,     // Might be string or number, let's use serde_json::Value or u64
    pub r#type: Option<String>, // "t" means track
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandCampAutocompleteResponse {
    pub results: Option<Vec<BandCampAutocompleteTrackObject>>,
}

pub async fn bandcamp_search(
    _player: &Player,
    query: &str,
    request_user: Option<TrackRequester>,
) -> UnresolvedSearchResult {
    let mut error = None;
    let mut tracks = vec![];

    // Minimal validation stub
    
    let url = format!("https://bandcamp.com/api/nusearch/2/autocomplete?q={}", urlencoding::encode(query));

    let client = reqwest::Client::builder()
        .user_agent("android-async-http/1.4.1 (http://loopj.com/android-async-http)")
        .build()
        .unwrap();

    let req = client.get(&url).header("Cookie", "$Version=1").send().await;

    match req {
        Ok(response) => {
            if !response.status().is_success() {
                error = Some(crate::types::utils::Exception {
                    severity: "error".to_string(),
                    message: format!("Bandcamp Error: {}", response.status()),
                    cause: "".to_string(),
                    cause_stack_trace: "".to_string(),
                });
            } else {
                match response.json::<BandCampAutocompleteResponse>().await {
                    Ok(data) => {
                        if let Some(results) = data.results {
                            for item in results {
                                if item.r#type.as_deref() == Some("t") {
                                    let uri = item.url.or(item.uri);
                                    let id = item.id.map(|x| x.to_string()).or_else(|| uri.clone().and_then(|u| u.split('/').last().map(String::from)));
                                    
                                    let info = UnresolvedTrackInfo {
                                        title: item.name.unwrap_or_default(),
                                        author: item.band_name,
                                        duration: None,
                                    };
                                    
                                    tracks.push(crate::types::track::UnresolvedTrack {
                                        encoded: None,
                                        info,
                                        plugin_info: None,
                                        user_data: None,
                                        requester: request_user.clone(),
                                    });
                                }
                            }
                        }
                    },
                    Err(_) => {
                        error = Some(crate::types::utils::Exception {
                            severity: "error".to_string(),
                            message: "Invalid JSON response from Bandcamp".to_string(),
                            cause: "".to_string(),
                            cause_stack_trace: "".to_string(),
                        });
                    }
                }
            }
        },
        Err(e) => {
            error = Some(crate::types::utils::Exception {
                severity: "error".to_string(),
                message: e.to_string(),
                cause: "".to_string(),
                cause_stack_trace: "".to_string(),
            });
        }
    }

    UnresolvedSearchResult {
        load_type: "search".to_string(),
        exception: error,
        plugin_info: crate::types::track::PluginInfo::default(),
        playlist: None,
        tracks,
    }
}
