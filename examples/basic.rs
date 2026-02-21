use lavalink_client_rs::manager::{LavalinkManager, LavalinkManagerOptions};
use lavalink_client_rs::node::LavalinkNodeOptions;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize standard logging
    tracing_subscriber::fmt::init();
    info!("Starting Lavalink Client RS test bot...");

    // Create manager
    let options = LavalinkManagerOptions {
        client_name: "LavalinkTestBot".to_string(),
        user_id: "123456789012345678".to_string(), // Dummy user ID for lavalink headers
        ..Default::default()
    };
    
    let (mut manager, mut rx) = LavalinkManager::new(options);

    // Create Node Options for a local Lavalink server
    let node_options = LavalinkNodeOptions {
        id: "LocalNode".to_string(),
        host: "localhost".to_string(),
        port: 2333,
        authorization: "heavy-cruiser-ibuki".to_string(),
        secure: Some(false),
        request_timeout: Some(10000),
        session_id: None,
    };

    info!("Adding node...");
    manager.node_manager.add_node(node_options, "123456789012345678".to_string(), "LavalinkTestBot".to_string()).await.unwrap();

    // Listen to events to know when it connects
    let search_manager = manager; 
    
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            info!("Received Event: {:?}", msg);
            match msg {
                lavalink_client_rs::types::events::LavalinkMessage::Ready { session_id, .. } => {
                    info!("Node is ready with session_id: {}", session_id);
                    
                    // We can try to search for a track once the node is ready
                    if let Some(node) = search_manager.node_manager.get_node("LocalNode") {
                        info!("Searching for 'ytsearch:hello'...");
                        match node.search("ytsearch:hello").await {
                            Ok(res) => {
                                info!("Search OK! Found {} tracks. LoadType: {}", res.tracks().len(), res.load_type);
                            },
                            Err(e) => {
                                error!("Search failed: {}", e);
                            }
                        }
                    } else {
                        error!("Node not found in node manager");
                    }
                },
                _ => {}
            }
        }
    });

    // Keeping the main thread alive
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");
    
    Ok(())
}
