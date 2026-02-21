use crate::node::LavalinkNode;
use crate::player::Player;
use crate::types::track::{Track, UnresolvedTrack};
use crate::types::node_link::*;
use crate::types::filters::*;
use async_trait::async_trait;

#[async_trait]
pub trait NodeLinkExt {
    async fn add_mixer_layer(&self, player: &Player, track_to_add: &Track, volume: i32) -> Result<AddMixerLayerResponse, String>;
    async fn list_mixer_layers(&self, player: &Player) -> Result<ListMixerLayersResponse, String>;
    async fn update_mixer_layer_volume(&self, player: &Player, mix_id: &str, volume: i32) -> Result<bool, String>;
    async fn remove_mixer_layer(&self, player: &Player, mix_id: &str) -> Result<bool, String>;
    
    // NodeLink filters
    async fn apply_echo_filter(&self, player: &mut Player, options: NodeLinkEchoFilter, disable: bool) -> Result<bool, String>;
    async fn apply_chorus_filter(&self, player: &mut Player, options: NodeLinkChorusFilter, disable: bool) -> Result<bool, String>;
    async fn apply_compressor_filter(&self, player: &mut Player, options: NodeLinkCompressorFilter, disable: bool) -> Result<bool, String>;
    async fn apply_high_pass_filter(&self, player: &mut Player, options: NodeLinkHighPassFilter, disable: bool) -> Result<bool, String>;
    async fn apply_phaser_filter(&self, player: &mut Player, options: NodeLinkPhaserFilter, disable: bool) -> Result<bool, String>;
    async fn apply_spatial_filter(&self, player: &mut Player, options: NodeLinkSpatialFilter, disable: bool) -> Result<bool, String>;
    async fn reset_node_link_filters(&self, player: &mut Player) -> Result<bool, String>;

    async fn node_link_lyrics(&self, player: &Player, track: Option<&Track>, language: &str) -> Result<NodeLinkLyrics, String>;
    async fn get_chapters(&self, player: &Player, track: Option<&Track>) -> Result<Vec<NodeLinkChapter>, String>;
    
    async fn get_connection_metrics(&self) -> Result<ConnectionMetricsResponse, String>;
    async fn get_direct_stream(&self, track: &Track) -> Result<DirectStreamResponse, String>;
    
    async fn change_audio_track_language(&self, player: &Player, language_audio_track_id: &str) -> Result<serde_json::Value, String>;
    
    async fn update_youtube_config(&self, refresh_token: Option<&str>, visitor_data: Option<&str>) -> Result<serde_json::Value, String>;
    async fn get_youtube_config(&self, validate: bool) -> Result<serde_json::Value, String>;
    async fn get_youtube_oauth(&self, refresh_token: &str) -> Result<YoutubeOAuthResponse, String>;
    async fn update_youtube_oauth(&self, refresh_token: &str) -> Result<YoutubeOAuthResponse, String>;
}

#[async_trait]
impl NodeLinkExt for LavalinkNode {
    async fn add_mixer_layer(&self, player: &Player, track_to_add: &Track, volume: i32) -> Result<AddMixerLayerResponse, String> {
        let sid = self.session_id.read().await.clone().ok_or("No session ID")?;
        let vol_str = format!("{:.2}", (volume as f64) / 100.0);
        let body = serde_json::json!({
            "track": {
                "encoded": track_to_add.encoded,
                "userData": track_to_add.user_data,
            },
            "volume": vol_str
        });
        
        let path = format!("/v4/sessions/{}/players/{}/mix", sid, player.guild_id);
        let res = self.request_with_body(reqwest::Method::POST, &path, &body).await?;
        serde_json::from_value(res).map_err(|e| e.to_string())
    }

    async fn list_mixer_layers(&self, player: &Player) -> Result<ListMixerLayersResponse, String> {
        let sid = self.session_id.read().await.clone().ok_or("No session ID")?;
        let path = format!("/v4/sessions/{}/players/{}/mix", sid, player.guild_id);
        let res = self.request(reqwest::Method::GET, &path).await?;
        serde_json::from_value(res).map_err(|e| e.to_string())
    }

    async fn update_mixer_layer_volume(&self, player: &Player, mix_id: &str, volume: i32) -> Result<bool, String> {
        let sid = self.session_id.read().await.clone().ok_or("No session ID")?;
        let vol_str = format!("{:.2}", (volume as f64) / 100.0);
        let body = serde_json::json!({ "volume": vol_str });
        let path = format!("/v4/sessions/{}/players/{}/mix/{}", sid, player.guild_id, mix_id);
        self.request_with_body(reqwest::Method::PATCH, &path, &body).await?;
        Ok(true)
    }

    async fn remove_mixer_layer(&self, player: &Player, mix_id: &str) -> Result<bool, String> {
        let sid = self.session_id.read().await.clone().ok_or("No session ID")?;
        let path = format!("/v4/sessions/{}/players/{}/mix/{}", sid, player.guild_id, mix_id);
        self.request(reqwest::Method::DELETE, &path).await?;
        Ok(true)
    }

    async fn apply_echo_filter(&self, player: &mut Player, options: NodeLinkEchoFilter, disable: bool) -> Result<bool, String> {
        if disable {
            player.filter_manager.data.echo = None;
        } else {
            player.filter_manager.data.echo = Some(options);
        }
        // In reality we'd apply the filter payload here to REST
        Ok(!disable)
    }

    async fn apply_chorus_filter(&self, player: &mut Player, options: NodeLinkChorusFilter, disable: bool) -> Result<bool, String> {
        if disable {
            player.filter_manager.data.chorus = None;
        } else {
            player.filter_manager.data.chorus = Some(options);
        }
        Ok(!disable)
    }

    async fn apply_compressor_filter(&self, player: &mut Player, options: NodeLinkCompressorFilter, disable: bool) -> Result<bool, String> {
        if disable {
            player.filter_manager.data.compressor = None;
        } else {
            player.filter_manager.data.compressor = Some(options);
        }
        Ok(!disable)
    }

    async fn apply_high_pass_filter(&self, player: &mut Player, options: NodeLinkHighPassFilter, disable: bool) -> Result<bool, String> {
        if disable {
            player.filter_manager.data.high_pass = None;
        } else {
            player.filter_manager.data.high_pass = Some(options);
        }
        Ok(!disable)
    }

    async fn apply_phaser_filter(&self, player: &mut Player, options: NodeLinkPhaserFilter, disable: bool) -> Result<bool, String> {
        if disable {
            player.filter_manager.data.phaser = None;
        } else {
            player.filter_manager.data.phaser = Some(options);
        }
        Ok(!disable)
    }

    async fn apply_spatial_filter(&self, player: &mut Player, options: NodeLinkSpatialFilter, disable: bool) -> Result<bool, String> {
        if disable {
            player.filter_manager.data.spatial = None;
        } else {
            player.filter_manager.data.spatial = Some(options);
        }
        Ok(!disable)
    }

    async fn reset_node_link_filters(&self, player: &mut Player) -> Result<bool, String> {
        player.filter_manager.data.spatial = None;
        player.filter_manager.data.echo = None;
        player.filter_manager.data.chorus = None;
        player.filter_manager.data.compressor = None;
        player.filter_manager.data.high_pass = None;
        player.filter_manager.data.phaser = None;
        Ok(true)
    }

    async fn node_link_lyrics(&self, player: &Player, track: Option<&Track>, language: &str) -> Result<NodeLinkLyrics, String> {
        let sid = self.session_id.read().await.clone().ok_or("No session ID")?;
        let encoded = track.and_then(|t| t.encoded.clone()).unwrap_or_default();
        let path = format!("/v4/sessions/{}/players/{}/lyrics?encodedTrack={}&lang={}", sid, player.guild_id, encoded, language);
        let res = self.request(reqwest::Method::GET, &path).await?;
        serde_json::from_value(res).map_err(|e| e.to_string())
    }

    async fn get_chapters(&self, player: &Player, track: Option<&Track>) -> Result<Vec<NodeLinkChapter>, String> {
        let sid = self.session_id.read().await.clone().ok_or("No session ID")?;
        let encoded = track.and_then(|t| t.encoded.clone()).unwrap_or_default();
        let path = format!("/v4/sessions/{}/players/{}/chapters?encodedTrack={}", sid, player.guild_id, encoded);
        let res = self.request(reqwest::Method::GET, &path).await?;
        serde_json::from_value(res).map_err(|e| e.to_string())
    }

    async fn get_connection_metrics(&self) -> Result<ConnectionMetricsResponse, String> {
        let res = self.request(reqwest::Method::GET, "/v4/connection").await?;
        serde_json::from_value(res).map_err(|e| e.to_string())
    }

    async fn get_direct_stream(&self, track: &Track) -> Result<DirectStreamResponse, String> {
        let encoded = track.encoded.clone().unwrap_or_default();
        let path = format!("/v4/trackstream?encodedTrack={}", encoded);
        let res = self.request(reqwest::Method::GET, &path).await?;
        serde_json::from_value(res).map_err(|e| e.to_string())
    }

    async fn change_audio_track_language(&self, player: &Player, language_audio_track_id: &str) -> Result<serde_json::Value, String> {
        let sid = self.session_id.read().await.clone().ok_or("No session ID")?;
        let body = serde_json::json!({
            "track": {
                "encoded": player.queue.current.as_ref().and_then(|t| t.encoded.clone()),
                "position": player.position(),
                "audioTrackId": language_audio_track_id
            }
        });
        let path = format!("/v4/sessions/{}/players/{}", sid, player.guild_id);
        self.request_with_body(reqwest::Method::PATCH, &path, &body).await
    }

    async fn update_youtube_config(&self, refresh_token: Option<&str>, visitor_data: Option<&str>) -> Result<serde_json::Value, String> {
        let body = serde_json::json!({
            "refreshToken": refresh_token,
            "visitorData": visitor_data
        });
        self.request_with_body(reqwest::Method::PATCH, "/v4/youtube/config", &body).await
    }

    async fn get_youtube_config(&self, validate: bool) -> Result<serde_json::Value, String> {
        let path = if validate { "/v4/youtube/config?validate=true" } else { "/v4/youtube/config" };
        self.request(reqwest::Method::GET, path).await
    }

    async fn get_youtube_oauth(&self, refresh_token: &str) -> Result<YoutubeOAuthResponse, String> {
        let path = format!("/v4/youtube/oauth?refreshToken={}", refresh_token);
        let res = self.request(reqwest::Method::GET, &path).await?;
        serde_json::from_value(res).map_err(|e| e.to_string())
    }

    async fn update_youtube_oauth(&self, refresh_token: &str) -> Result<YoutubeOAuthResponse, String> {
        let body = serde_json::json!({ "refreshToken": refresh_token });
        let res = self.request_with_body(reqwest::Method::POST, "/v4/youtube/oauth", &body).await?;
        serde_json::from_value(res).map_err(|e| e.to_string())
    }
}
