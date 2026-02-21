use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, debug};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

use crate::types::events::{LavalinkMessage, SearchResult, LavalinkPlayer};
use crate::types::player::LavalinkPlayOptions;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LavalinkNodeOptions {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub authorization: String,
    pub secure: Option<bool>,
    pub request_timeout: Option<u64>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStats {
    pub players: i32,
    pub playing_players: i32,
    pub uptime: i64,
    pub memory: NodeMemoryStats,
    pub cpu: NodeCpuStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeMemoryStats {
    pub free: u64,
    pub used: u64,
    pub allocated: u64,
    pub reservable: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCpuStats {
    pub cores: i32,
    pub system_load: f32,
    pub lavalink_load: f32,
}

pub struct LavalinkNode {
    pub id: String,
    pub options: LavalinkNodeOptions,
    pub session_id: RwLock<Option<String>>,
    pub stats: RwLock<Option<NodeStats>>,
    pub connected: RwLock<bool>,
    reqwest_client: reqwest::Client,
    // Channel to push events to the NodeManager
    pub event_sender: mpsc::Sender<LavalinkMessage>,
}

impl LavalinkNode {
    pub fn new(options: LavalinkNodeOptions, event_sender: mpsc::Sender<LavalinkMessage>) -> Self {
        let initial_session = options.session_id.clone();
        Self {
            id: options.id.clone(),
            options,
            session_id: RwLock::new(initial_session),
            stats: RwLock::new(None),
            connected: RwLock::new(false),
            reqwest_client: reqwest::Client::new(),
            event_sender,
        }
    }

    pub fn get_rest_url(&self) -> String {
        let protocol = if self.options.secure.unwrap_or(false) { "https" } else { "http" };
        format!("{}://{}:{}", protocol, self.options.host, self.options.port)
    }

    pub fn get_ws_url(&self) -> String {
        let protocol = if self.options.secure.unwrap_or(false) { "wss" } else { "ws" };
        format!("{}://{}:{}", protocol, self.options.host, self.options.port)
    }

    pub async fn connect(node_arc: Arc<Self>, user_id: String, client_name: String) -> Result<(), String> {
        let ws_url = format!("{}/v4/websocket", node_arc.get_ws_url());
        
        let mut request = ws_url.into_client_request().map_err(|e| e.to_string())?;
        
        let headers = request.headers_mut();
        headers.insert("Authorization", HeaderValue::from_str(&node_arc.options.authorization).unwrap());
        headers.insert("User-Id", HeaderValue::from_str(&user_id).unwrap());
        headers.insert("Client-Name", HeaderValue::from_str(&client_name).unwrap());
        
        let current_session = node_arc.session_id.read().await.clone();
        if let Some(session_id) = current_session {
            headers.insert("Session-Id", HeaderValue::from_str(&session_id).unwrap());
        }

        let (ws_stream, _) = connect_async(request)
            .await
            .map_err(|e| format!("Failed to connect to Websocket: {}", e))?;
            
        info!("Connected to Lavalink Node {}", node_arc.id);
        *node_arc.connected.write().await = true;

        let (mut write, mut read) = ws_stream.split();
        
        let n = node_arc.clone();
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        debug!("Received WebSocket message: {}", text);
                        match serde_json::from_str::<LavalinkMessage>(&text) {
                            Ok(lavalink_msg) => {
                                match &lavalink_msg {
                                    LavalinkMessage::Ready { session_id, .. } => {
                                        *n.session_id.write().await = Some(session_id.clone());
                                        info!("Lavalink Node {} is READY with session {}", n.id, session_id);
                                    },
                                    LavalinkMessage::Stats(stats) => {
                                        *n.stats.write().await = Some(stats.clone());
                                    },
                                    _ => {}
                                }
                                
                                // Forward to the Manager event bus
                                let _ = n.event_sender.send(lavalink_msg).await;
                            },
                            Err(e) => {
                                error!("Failed to deserialize Lavalink message: {} | Data: {}", e, text);
                            }
                        }
                    },
                    Ok(Message::Close(c)) => {
                        warn!("WebSocket closed by node {}: {:?}", n.id, c);
                        *n.connected.write().await = false;
                        break;
                    },
                    Err(e) => {
                        error!("WebSocket error on node {}: {}", n.id, e);
                        *n.connected.write().await = false;
                        break;
                    },
                    _ => {}
                }
            }
            warn!("Event loop terminated for node {}", n.id);
        });

        Ok(())
    }

    /// Update the Player on the Lavalink Server
    pub async fn update_player(&self, guild_id: &str, no_replace: bool, update_data: &LavalinkPlayOptions) -> Result<LavalinkPlayer, String> {
        let session = self.session_id.read().await.clone();
        if session.is_none() {
            return Err(format!("Node {} is not ready yet: No Session ID", self.id));
        }
        
        let url = format!("{}/v4/sessions/{}/players/{}?noReplace={}", self.get_rest_url(), session.unwrap(), guild_id, no_replace);
        
        let req = self.reqwest_client
            .patch(&url)
            .header("Authorization", &self.options.authorization)
            .json(update_data)
            .send()
            .await
            .map_err(|e| e.to_string())?;
            
        if !req.status().is_success() {
            let status = req.status();
            let body = req.text().await.unwrap_or_default();
            return Err(format!("Failed to update player ({}): {}", status, body));
        }

        let player = req.json::<LavalinkPlayer>().await.map_err(|e| e.to_string())?;
        Ok(player)
    }

    /// Destroy the Player on the Lavalink Server
    pub async fn destroy_player(&self, guild_id: &str) -> Result<(), String> {
        let session = self.session_id.read().await.clone();
        if session.is_none() {
            return Err(format!("Node {} is not ready yet: No Session ID", self.id));
        }
        
        let url = format!("{}/v4/sessions/{}/players/{}", self.get_rest_url(), session.unwrap(), guild_id);
        let req = self.reqwest_client
            .delete(&url)
            .header("Authorization", &self.options.authorization)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !req.status().is_success() && req.status() != 204 {
            return Err(format!("Failed to destroy player ({})", req.status()));
        }
        
        Ok(())
    }

    /// Load tracks / Search via loadtracks endpoint
    pub async fn search(&self, query: &str) -> Result<SearchResult, String> {
        let url = format!("{}/v4/loadtracks?identifier={}", self.get_rest_url(), urlencoding::encode(query));
        
        let req = self.reqwest_client
            .get(&url)
            .header("Authorization", &self.options.authorization)
            .send()
            .await
            .map_err(|e| e.to_string())?;
            
        if !req.status().is_success() {
            let status = req.status();
            let text = req.text().await.unwrap_or_default();
            return Err(format!("Failed to load tracks: {} | {}", status, text));
        }
        
        let body = req.text().await.map_err(|e| e.to_string())?;
        serde_json::from_str::<SearchResult>(&body)
            .map_err(|e| format!("Deserialize error: {} | body snippet: {}", e, &body[..body.len().min(300)]))
    }

    pub async fn check_status(&self) -> Result<String, String> {
        let url = format!("{}/v4/info", self.get_rest_url());
        let res = self.reqwest_client
            .get(&url)
            .header("Authorization", &self.options.authorization)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        res.text().await.map_err(|e| e.to_string())
    }

    pub async fn fetch_all_players(&self) -> Result<Vec<LavalinkPlayer>, String> {
        let session = self.session_id.read().await.clone();
        if session.is_none() {
            return Err(format!("Node {} is not ready yet: No Session ID", self.id));
        }
        let url = format!("{}/v4/sessions/{}/players", self.get_rest_url(), session.unwrap());
        let res = self.reqwest_client.get(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        res.json::<Vec<LavalinkPlayer>>().await.map_err(|e| e.to_string())
    }

    pub async fn fetch_player(&self, guild_id: &str) -> Result<LavalinkPlayer, String> {
        let session = self.session_id.read().await.clone();
        if session.is_none() {
            return Err(format!("Node {} is not ready yet: No Session ID", self.id));
        }
        let url = format!("{}/v4/sessions/{}/players/{}", self.get_rest_url(), session.unwrap(), guild_id);
        let res = self.reqwest_client.get(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        res.json::<LavalinkPlayer>().await.map_err(|e| e.to_string())
    }

    pub async fn update_session(&self, resuming: Option<bool>, timeout: Option<u64>) -> Result<serde_json::Value, String> {
        let session = self.session_id.read().await.clone();
        if session.is_none() {
            return Err(format!("Node {} is not ready yet: No Session ID", self.id));
        }
        let url = format!("{}/v4/sessions/{}", self.get_rest_url(), session.unwrap());
        let mut body = serde_json::Map::new();
        if let Some(r) = resuming { body.insert("resuming".to_string(), serde_json::Value::Bool(r)); }
        if let Some(t) = timeout { body.insert("timeout".to_string(), serde_json::Value::Number(serde_json::Number::from(t))); }
        let res = self.reqwest_client.patch(&url).header("Authorization", &self.options.authorization).json(&body).send().await.map_err(|e| e.to_string())?;
        res.json::<serde_json::Value>().await.map_err(|e| e.to_string())
    }

    pub async fn decode_single_track(&self, encoded: &str) -> Result<crate::types::track::LavalinkTrack, String> {
        let url = format!("{}/v4/decodetrack?encodedTrack={}", self.get_rest_url(), urlencoding::encode(encoded));
        let res = self.reqwest_client.get(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        res.json::<crate::types::track::LavalinkTrack>().await.map_err(|e| e.to_string())
    }

    pub async fn decode_multiple_tracks(&self, encodeds: Vec<String>) -> Result<Vec<crate::types::track::LavalinkTrack>, String> {
        let url = format!("{}/v4/decodetracks", self.get_rest_url());
        let res = self.reqwest_client.post(&url).header("Authorization", &self.options.authorization).json(&encodeds).send().await.map_err(|e| e.to_string())?;
        res.json::<Vec<crate::types::track::LavalinkTrack>>().await.map_err(|e| e.to_string())
    }

    pub async fn get_lyrics(&self, track: &crate::types::track::Track, skip_track_source: bool) -> Result<serde_json::Value, String> {
        let encoded = track.encoded.as_deref().unwrap_or_default();
        let url = format!("{}/v4/lyrics?track={}&skipTrackSource={}", self.get_rest_url(), encoded, skip_track_source);
        let res = self.reqwest_client.get(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        res.json::<serde_json::Value>().await.map_err(|e| e.to_string())
    }

    pub async fn get_current_lyrics(&self, guild_id: &str, skip_track_source: bool) -> Result<serde_json::Value, String> {
        let session = self.session_id.read().await.clone();
        if session.is_none() { return Err("No Session".to_string()); }
        let url = format!("{}/v4/sessions/{}/players/{}/track/lyrics?skipTrackSource={}", self.get_rest_url(), session.unwrap(), guild_id, skip_track_source);
        let res = self.reqwest_client.get(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        res.json::<serde_json::Value>().await.map_err(|e| e.to_string())
    }

    pub async fn subscribe_lyrics(&self, guild_id: &str) -> Result<(), String> {
        let session = self.session_id.read().await.clone();
        if session.is_none() { return Err("No Session".to_string()); }
        let url = format!("{}/v4/sessions/{}/players/{}/lyrics/subscribe", self.get_rest_url(), session.unwrap(), guild_id);
        self.reqwest_client.post(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn unsubscribe_lyrics(&self, guild_id: &str) -> Result<(), String> {
        let session = self.session_id.read().await.clone();
        if session.is_none() { return Err("No Session".to_string()); }
        let url = format!("{}/v4/sessions/{}/players/{}/lyrics/subscribe", self.get_rest_url(), session.unwrap(), guild_id);
        self.reqwest_client.delete(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn fetch_stats(&self) -> Result<NodeStats, String> {
        let url = format!("{}/v4/stats", self.get_rest_url());
        let res = self.reqwest_client.get(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        res.json::<NodeStats>().await.map_err(|e| e.to_string())
    }

    pub async fn fetch_connection_metrics(&self) -> Result<serde_json::Value, String> {
        let url = format!("{}/v4/connection", self.get_rest_url());
        let res = self.reqwest_client.get(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        res.json::<serde_json::Value>().await.map_err(|e| e.to_string())
    }

    pub async fn fetch_version(&self) -> Result<String, String> {
        let url = format!("{}/version", self.get_rest_url());
        let res = self.reqwest_client.get(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        res.text().await.map_err(|e| e.to_string())
    }

    pub async fn fetch_info(&self) -> Result<serde_json::Value, String> {
        let url = format!("{}/v4/info", self.get_rest_url());
        let res = self.reqwest_client.get(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        res.json::<serde_json::Value>().await.map_err(|e| e.to_string())
    }

    pub async fn route_planner_status(&self) -> Result<serde_json::Value, String> {
        let url = format!("{}/v4/routeplanner/status", self.get_rest_url());
        let res = self.reqwest_client.get(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        res.json::<serde_json::Value>().await.map_err(|e| e.to_string())
    }

    pub async fn route_planner_unmark_failed_address(&self, address: &str) -> Result<(), String> {
        let url = format!("{}/v4/routeplanner/free/address", self.get_rest_url());
        self.reqwest_client.post(&url).header("Authorization", &self.options.authorization).json(&serde_json::json!({ "address": address })).send().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn route_planner_unmark_all_failed_addresses(&self) -> Result<(), String> {
        let url = format!("{}/v4/routeplanner/free/all", self.get_rest_url());
        self.reqwest_client.post(&url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn request(&self, method: reqwest::Method, path: &str) -> Result<serde_json::Value, String> {
        let url = format!("{}{}", self.get_rest_url(), path);
        let req = self.reqwest_client.request(method, &url).header("Authorization", &self.options.authorization).send().await.map_err(|e| e.to_string())?;
        req.json::<serde_json::Value>().await.map_err(|e| e.to_string())
    }

    pub async fn request_with_body(&self, method: reqwest::Method, path: &str, body: &serde_json::Value) -> Result<serde_json::Value, String> {
        let url = format!("{}{}", self.get_rest_url(), path);
        let req = self.reqwest_client.request(method, &url).header("Authorization", &self.options.authorization).json(body).send().await.map_err(|e| e.to_string())?;
        req.json::<serde_json::Value>().await.map_err(|e| e.to_string())
    }
}
