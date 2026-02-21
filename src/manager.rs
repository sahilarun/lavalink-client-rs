use crate::node_manager::NodeManager;
use crate::utils::ManagerUtils;
use crate::player::Player;
use crate::types::player::PlayerOptions;
use crate::types::events::LavalinkMessage;
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};

#[derive(Debug, Clone)]
pub struct LavalinkManagerOptions {
    pub send_to_shard: bool, 
    pub auto_skip: bool,
    pub emit_new_songs_only: bool,
    pub user_id: String,
    pub client_name: String,
}

impl Default for LavalinkManagerOptions {
    fn default() -> Self {
        Self {
            send_to_shard: false,
            auto_skip: true,
            emit_new_songs_only: false,
            user_id: String::new(),
            client_name: "lavalink-client-rs".to_string(),
        }
    }
}

pub struct LavalinkManager {
    pub options: LavalinkManagerOptions,
    pub node_manager: NodeManager,
    pub utils: ManagerUtils,
    pub players: RwLock<HashMap<String, Player>>, 
    // Usually an event bus / emitter here, for now we will just process to stdout.
}

impl LavalinkManager {
    pub fn new(options: LavalinkManagerOptions) -> (Self, mpsc::Receiver<LavalinkMessage>) {
        let (tx, rx) = mpsc::channel(100);
        
        let manager = Self {
            options: options.clone(),
            node_manager: NodeManager::new(tx),
            utils: ManagerUtils::new("ytsearch".to_string()),
            players: RwLock::new(HashMap::new()),
        };
        
        (manager, rx)
    }
    
    pub async fn create_player(&self, options: PlayerOptions) -> Result<Player, String> {
        let mut players = self.players.write().await;
        
        if players.contains_key(&options.guild_id) {
            return Err(format!("Player for guild {} already exists", options.guild_id));
        }
        
        let guild_id = options.guild_id.clone();
        
        if let Some(node) = self.node_manager.least_used_node() {
            let player = Player::new(options, node, self.node_manager.event_sender.clone(), false);
            players.insert(guild_id, player.clone());
            Ok(player)
        } else {
            return Err("No available Lavalink node to assign".to_string());
        }
    }
    
    pub async fn get_player(&self, guild_id: &str) -> Option<Player> {
        let players = self.players.read().await;
        players.get(guild_id).cloned() 
    }
    
    pub async fn delete_player(&self, guild_id: &str) -> bool {
        let mut players = self.players.write().await;
        players.remove(guild_id).is_some()
    }
    
    pub async fn voice_server_update(&self, guild_id: &str, _endpoint: &str, _session_id: &str, _token: &str) -> Result<(), String> {
        let mut players = self.players.write().await;
        if let Some(player) = players.get_mut(guild_id) {
            player.voice_state.server_deaf = false; 
        }
        Ok(())
    }
}
