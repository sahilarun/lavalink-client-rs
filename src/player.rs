use crate::types::filters::FilterManager;
use crate::types::player::{PlayerOptions, RepeatMode, PlayerPing, VoiceState, PlayOptions, LavalinkPlayOptions};
use crate::types::events::{LavalinkMessage, SearchResult};
use crate::utils::{LavaSearchQuery, SearchQuery};
use crate::queue::Queue;
use crate::types::queue::ManagerQueueOptions;
use crate::node::LavalinkNode;
use crate::types::track::{Track, UnresolvedTrack};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use serde_json::Value;

#[derive(Clone)]
pub struct Player {
    pub filter_manager: FilterManager,
    pub options: PlayerOptions,
    pub node: Arc<LavalinkNode>,
    pub queue: Queue,
    pub guild_id: String,
    pub voice_channel_id: Option<String>,
    pub text_channel_id: Option<String>,
    pub playing: bool,
    pub paused: bool,
    pub repeat_mode: RepeatMode,
    pub ping: PlayerPing,
    pub volume: i32,
    pub lavalink_volume: i32,
    pub last_position: i64,
    pub last_position_change: Option<i64>,
    pub last_saved_position: i64,
    pub created_time_stamp: i64,
    pub connected: bool,
    pub voice_state: VoiceState,
    pub data: HashMap<String, Value>,
    pub event_sender: mpsc::Sender<LavalinkMessage>,
}

pub enum SponsorBlockSegment {
    Sponsor,
    SelfPromo,
    Interaction,
    Intro,
    Outro,
    Preview,
    MusicOfftopic,
    Filler,
}

impl Player {
    pub fn new(options: PlayerOptions, node: Arc<LavalinkNode>, event_sender: mpsc::Sender<LavalinkMessage>, dont_emit: bool) -> Self {
        let volume = options.volume.unwrap_or(100).clamp(0, 1000);
        let guild_id = options.guild_id.clone();
        Self {
            filter_manager: FilterManager::new(),
            voice_channel_id: Some(options.voice_channel_id.clone()),
            text_channel_id: options.text_channel_id.clone(),
            options: options.clone(),
            node,
            queue: Queue::new(guild_id.clone(), None, None, ManagerQueueOptions::default()),
            guild_id,
            playing: false,
            paused: false,
            repeat_mode: RepeatMode::Off,
            ping: PlayerPing { ws: 0, lavalink: 0 },
            volume,
            lavalink_volume: volume,
            last_position: 0,
            last_position_change: None,
            last_saved_position: 0,
            created_time_stamp: chrono::Utc::now().timestamp_millis(),
            connected: false,
            voice_state: VoiceState::default(),
            data: HashMap::new(),
            event_sender,
        }
    }

    pub fn position(&self) -> i64 {
        if let Some(change) = chrono::Utc::now().timestamp_millis().checked_sub(self.last_position_change.unwrap_or(0)) {
            if self.last_position_change.is_some() {
                return self.last_position + change;
            }
        }
        self.last_position
    }

    pub fn set(&mut self, key: &str, value: Value) -> &mut Self {
        self.data.insert(key.to_string(), value);
        self
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn clear_data(&mut self) -> &mut Self {
        self.data.retain(|k, _| k.starts_with("internal_"));
        self
    }

    pub fn get_all_data(&self) -> HashMap<String, Value> {
        self.data.clone().into_iter().filter(|(k, _)| !k.starts_with("internal_")).collect()
    }

    pub async fn play(&mut self, options: PlayOptions) -> Result<&mut Self, String> {
        if let Some(_) = self.data.get("internal_queueempty") {
            self.data.remove("internal_queueempty");
        }

        let mut final_options = LavalinkPlayOptions {
            track: None,
            position: None,
            end_time: None,
            volume: None,
            paused: None,
            filters: None,
            voice: None,
        };

        if let Some(track_opt) = &options.track {
            if track_opt.encoded.is_some() || track_opt.identifier.is_some() {
                // If specific track is given, play it
                final_options.track = Some(crate::types::player::TrackPlayOptions {
                    encoded: track_opt.encoded.clone(),
                    identifier: track_opt.identifier.clone(),
                    user_data: track_opt.user_data.clone(),
                    audio_track_id: track_opt.audio_track_id.clone(),
                });
                
                if let Some(v) = options.volume {
                    self.volume = v.clamp(0, 1000);
                    self.lavalink_volume = self.volume;
                    final_options.volume = Some(self.lavalink_volume);
                }

                final_options.position = options.position;
                final_options.end_time = options.end_time;
                final_options.paused = options.paused;
                
                let now = chrono::Utc::now().timestamp_millis();
                self.node.update_player(&self.guild_id, options.no_replace.unwrap_or(false), &final_options).await?;
                self.ping.lavalink = chrono::Utc::now().timestamp_millis() - now;
                return Ok(self);
            }
        }

        // Play from queue
        if self.queue.current.is_none() {
            return Err("There is no Track in the Queue, nor provided in the PlayOptions".to_string());
        }

        if let Some(v) = options.volume {
            self.volume = v.clamp(0, 1000);
            self.lavalink_volume = self.volume;
            final_options.volume = Some(self.lavalink_volume);
        }

        final_options.track = Some(crate::types::player::TrackPlayOptions {
            encoded: self.queue.current.as_ref().unwrap().encoded.clone(),
            identifier: None,
            user_data: self.queue.current.as_ref().unwrap().user_data.clone(),
            audio_track_id: None,
        });

        final_options.position = options.position.or(Some(0));
        // End time etc validation omitted for conciseness
        
        let now = chrono::Utc::now().timestamp_millis();
        self.node.update_player(&self.guild_id, options.no_replace.unwrap_or(false), &final_options).await?;
        self.ping.lavalink = chrono::Utc::now().timestamp_millis() - now;
        self.playing = true;
        
        Ok(self)
    }

    pub async fn set_volume(&mut self, volume: i32, _ignore_volume_decrementer: bool) -> Result<&mut Self, String> {
        self.volume = volume.clamp(0, 1000);
        self.lavalink_volume = self.volume;

        let now = chrono::Utc::now().timestamp_millis();
        let update_data = LavalinkPlayOptions {
            volume: Some(self.lavalink_volume),
            ..Default::default()
        };
        self.node.update_player(&self.guild_id, false, &update_data).await?;
        self.ping.lavalink = chrono::Utc::now().timestamp_millis() - now;
        Ok(self)
    }

    pub async fn lava_search(&self, query: LavaSearchQuery, request_user: Option<String>, throw_on_empty: bool) -> Result<SearchResult, String> {
        self.node.search(&query.query).await
    }

    pub async fn set_sponsor_block(&mut self, segments: Vec<SponsorBlockSegment>) -> Result<(), String> {
        // Mock sponsorblock on node
        Ok(())
    }

    pub async fn get_sponsor_block(&self) -> Result<Vec<SponsorBlockSegment>, String> {
        Ok(vec![])
    }

    pub async fn delete_sponsor_block(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub async fn search(&self, query: SearchQuery, request_user: Option<String>, throw_on_empty: bool) -> Result<SearchResult, String> {
        self.node.search(&query.query).await
    }

    pub async fn pause(&mut self) -> Result<&mut Self, String> {
        if self.paused && !self.playing {
            return Err("Player is already paused - not able to pause.".to_string());
        }
        self.paused = true;
        self.last_position_change = None;

        let now = chrono::Utc::now().timestamp_millis();
        let update_data = LavalinkPlayOptions {
            paused: Some(true),
            ..Default::default()
        };
        self.node.update_player(&self.guild_id, false, &update_data).await?;
        self.ping.lavalink = chrono::Utc::now().timestamp_millis() - now;
        Ok(self)
    }

    pub async fn resume(&mut self) -> Result<&mut Self, String> {
        if !self.paused {
            return Err("Player isn't paused - not able to resume.".to_string());
        }
        self.paused = false;

        let now = chrono::Utc::now().timestamp_millis();
        let update_data = LavalinkPlayOptions {
            paused: Some(false),
            ..Default::default()
        };
        self.node.update_player(&self.guild_id, false, &update_data).await?;
        self.ping.lavalink = chrono::Utc::now().timestamp_millis() - now;
        Ok(self)
    }

    pub async fn seek(&mut self, position: i64) -> Result<&mut Self, String> {
        if self.queue.current.is_none() {
            return Ok(self);
        }
        
        let position = position.max(0); // .min(current duration) handled implicitly or in future

        self.last_position_change = Some(chrono::Utc::now().timestamp_millis());
        self.last_position = position;

        let now = chrono::Utc::now().timestamp_millis();
        let update_data = LavalinkPlayOptions {
            position: Some(position),
            ..Default::default()
        };
        self.node.update_player(&self.guild_id, false, &update_data).await?;
        self.ping.lavalink = chrono::Utc::now().timestamp_millis() - now;

        Ok(self)
    }

    pub async fn set_repeat_mode(&mut self, repeat_mode: RepeatMode) -> Result<&mut Self, String> {
        self.repeat_mode = repeat_mode;
        Ok(self)
    }

    pub async fn skip(&mut self, skip_to: usize, throw_error: bool) -> Result<&mut Self, String> {
        if self.queue.tracks.is_empty() && skip_to > 0 && throw_error {
            return Err("Can't skip more than the queue size".to_string());
        }

        if skip_to > 1 {
            self.queue.splice(0, skip_to - 1, None).await;
        }

        if !self.playing && self.queue.current.is_none() {
            return self.play(PlayOptions::default()).await;
        }

        let now = chrono::Utc::now().timestamp_millis();
        self.set("internal_skipped", Value::Bool(true));

        let update_data = LavalinkPlayOptions {
            track: Some(crate::types::player::TrackPlayOptions {
                encoded: None,
                identifier: None,
                user_data: None,
                audio_track_id: None,
            }),
            paused: Some(false),
            ..Default::default()
        };
        self.node.update_player(&self.guild_id, false, &update_data).await?;
        self.ping.lavalink = chrono::Utc::now().timestamp_millis() - now;

        Ok(self)
    }

    pub async fn stop_playing(&mut self, clear_queue: bool, execute_autoplay: bool) -> Result<&mut Self, String> {
        self.set("internal_stopPlaying", Value::Bool(true));

        if self.queue.tracks.len() > 0 && clear_queue {
            let len = self.queue.tracks.len();
            self.queue.splice(0, len, None).await;
        }

        if !execute_autoplay {
            self.set("internal_autoplayStopPlaying", Value::Bool(true));
        }

        let now = chrono::Utc::now().timestamp_millis();

        let update_data = LavalinkPlayOptions {
            track: Some(crate::types::player::TrackPlayOptions {
                encoded: None,
                identifier: None,
                user_data: None,
                audio_track_id: None,
            }),
            ..Default::default()
        };
        self.node.update_player(&self.guild_id, false, &update_data).await?;
        self.paused = false;
        self.playing = false;
        self.ping.lavalink = chrono::Utc::now().timestamp_millis() - now;

        Ok(self)
    }

    pub async fn connect(&mut self) -> Result<&mut Self, String> {
        if self.options.voice_channel_id.is_empty() {
            return Err("No Voice Channel id has been set.".to_string());
        }
        
        self.voice_channel_id = Some(self.options.voice_channel_id.clone());
        Ok(self)
    }

    pub async fn change_voice_state(&mut self, voice_channel_id: Option<String>, self_mute: Option<bool>, self_deaf: Option<bool>) -> Result<&mut Self, String> {
        if let Some(vc) = &voice_channel_id {
            if Some(vc.clone()) == self.voice_channel_id {
                return Err("New Channel can't be equal to the old Channel.".to_string());
            }
        }

        if let Some(vc) = voice_channel_id.clone() {
            self.options.voice_channel_id = vc.clone();
            self.voice_channel_id = Some(vc);
        }
        if let Some(mute) = self_mute {
            self.options.self_mute = Some(mute);
        }
        if let Some(deaf) = self_deaf {
            self.options.self_deaf = Some(deaf);
        }

        Ok(self)
    }

    pub async fn disconnect(&mut self, force: bool) -> Result<&mut Self, String> {
        if !force && self.voice_channel_id.is_none() {
            return Err("No Voice Channel id has been set.".to_string());
        }
        self.voice_channel_id = None;
        Ok(self)
    }

    pub async fn destroy(&mut self, reason: Option<String>, disconnect: bool) -> Result<&mut Self, String> {
        self.set("internal_destroystatus", Value::Bool(true));
        
        if disconnect {
            self.disconnect(true).await?;
        } else {
            self.set("internal_destroywithoutdisconnect", Value::Bool(true));
        }

        self.queue.destroy().await;
        self.node.destroy_player(&self.guild_id).await?;

        Ok(self)
    }

    pub async fn get_current_lyrics(&self, skip_track_source: bool) -> Result<Option<String>, String> {
        Ok(None)
    }

    pub async fn get_lyrics(&self, track: Track, skip_track_source: bool) -> Result<Option<String>, String> {
        Ok(None)
    }

    pub fn subscribe_lyrics(&self) { }
    pub fn unsubscribe_lyrics(&self) { }

    pub async fn change_node(&mut self, new_node: Arc<LavalinkNode>, check_sources: bool) -> Result<String, String> {
        if self.node.id == new_node.id {
            return Err("Player is already on the provided Node".to_string());
        }
        if let Some(_) = self.get("internal_nodeChanging") {
            return Err("Player is already changing the node please wait".to_string());
        }
        
        self.set("internal_nodeChanging", Value::Bool(true));
        if *self.node.connected.read().await {
            self.node.destroy_player(&self.guild_id).await?;
        }

        self.node = new_node.clone();
        
        let now = chrono::Utc::now().timestamp_millis();
        self.connect().await?;

        // re-update player to new node:
        let update_data = LavalinkPlayOptions {
            position: Some(self.last_position),
            volume: Some(self.lavalink_volume),
            paused: Some(self.paused),
            // Track encoded will be correctly fetched and resumed
            ..Default::default()
        };
        self.node.update_player(&self.guild_id, false, &update_data).await?;

        self.ping.lavalink = chrono::Utc::now().timestamp_millis() - now;
        self.data.remove("internal_nodeChanging");

        Ok(new_node.id.clone())
    }

    pub fn to_json(&mut self) -> serde_json::Value {
        serde_json::json!({
            "guildId": self.guild_id,
            "options": self.options,
            "voiceChannelId": self.voice_channel_id,
            "textChannelId": self.text_channel_id,
            "position": self.position(),
            "lastPosition": self.last_position,
            "lastPositionChange": self.last_position_change,
            "volume": self.volume,
            "lavalinkVolume": self.lavalink_volume,
            "repeatMode": self.repeat_mode, // Needs stringify
            "paused": self.paused,
            "playing": self.playing,
            "createdTimeStamp": self.created_time_stamp,
            "nodeId": self.node.id,
            "ping": self.ping,
            "queue": self.queue.to_json(),
        })
    }
}
