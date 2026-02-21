use crate::node::{LavalinkNode, LavalinkNodeOptions};
use crate::types::events::LavalinkMessage;
use std::collections::HashMap;
use tokio::sync::mpsc;
use std::sync::Arc;

pub struct NodeManager {
    pub nodes: HashMap<String, Arc<LavalinkNode>>,
    pub event_sender: mpsc::Sender<LavalinkMessage>,
}

impl NodeManager {
    pub fn new(event_sender: mpsc::Sender<LavalinkMessage>) -> Self {
        Self {
            nodes: HashMap::new(),
            event_sender,
        }
    }

    pub async fn add_node(&mut self, options: LavalinkNodeOptions, user_id: String, client_name: String) -> Result<(), String> {
        if self.nodes.contains_key(&options.id) {
            return Err(format!("Node with id {} already exists", options.id));
        }
        
        let id = options.id.clone();
        let node = Arc::new(LavalinkNode::new(options, self.event_sender.clone()));
        
        LavalinkNode::connect(node.clone(), user_id, client_name).await?;
        
        self.nodes.insert(id, node);
        Ok(())
    }

    pub fn get_node(&self, id: &str) -> Option<Arc<LavalinkNode>> {
        self.nodes.get(id).cloned()
    }

    pub fn remove_node(&mut self, id: &str) -> bool {
        self.nodes.remove(id).is_some()
    }

    pub fn least_used_node(&self) -> Option<Arc<LavalinkNode>> {
        // Find node with fewest players. Simple stub.
        self.nodes.values().next().cloned()
    }
}
