# 🚀 Getting Started

This guide walks you through setting up `lavalink-client-rs` from scratch.

> [!NOTE]
> This client was built primarily for **[Tranquil](https://github.com/sahilarun/tranquil)** — a custom Lavalink-compatible audio server — and also works with standard **Lavalink v4**.

---

## Prerequisites

- Rust (edition 2021+)
- A running Lavalink v4 (or [Tranquil](https://github.com/sahilarun/tranquil)) server
- A Discord bot token + application ID

---

## 1. Add the Dependency

```toml
# Cargo.toml
[dependencies]
lavalink-client-rs = { git = "https://github.com/sahilarun/lavalink-client-rs" }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

## 2. Create the Manager

`LavalinkManager::new()` returns two things:
- The manager itself (holds node + player state)
- An `mpsc::Receiver<LavalinkMessage>` for all events

```rust
use lavalink_client_rs::manager::{LavalinkManager, LavalinkManagerOptions};

let options = LavalinkManagerOptions {
    user_id: "YOUR_DISCORD_BOT_USER_ID".to_string(),
    client_name: "MyBot/1.0".to_string(),
    auto_skip: true,
    ..Default::default()
};

let (manager, mut rx) = LavalinkManager::new(options);
```

---

## 3. Add a Node

```rust
use lavalink_client_rs::node::LavalinkNodeOptions;

manager.node_manager.add_node(
    LavalinkNodeOptions {
        id: "MainNode".to_string(),
        host: "localhost".to_string(),
        port: 2333,
        authorization: "youshallnotpass".to_string(),
        secure: Some(false),
        request_timeout: Some(10_000),
        session_id: None,
    },
    "YOUR_DISCORD_BOT_USER_ID".to_string(),
    "MyBot/1.0".to_string(),
).await?;
```

This opens a WebSocket connection to the node. Once connected you'll receive a `LavalinkMessage::Ready` event.

---

## 4. Handle Events

```rust
use lavalink_client_rs::types::events::LavalinkMessage;
use tracing::{info, error};

tokio::spawn(async move {
    while let Some(event) = rx.recv().await {
        match event {
            LavalinkMessage::Ready { session_id, resumed } => {
                info!("Node ready! Session: {} (resumed={})", session_id, resumed);
            }
            LavalinkMessage::Stats(stats) => {
                info!("Node stats — players: {}", stats.players);
            }
            _ => {}
        }
    }
});

// Keep the main thread alive
tokio::signal::ctrl_c().await?;
```

---

## 5. Run the Example

The bundled example connects to a local Lavalink server and performs a YouTube search:

```bash
cargo run --example basic
```

Expected output:
```
INFO  Starting Lavalink Client RS test bot...
INFO  Adding node...
INFO  Connected to Lavalink Node LocalNode
INFO  Node is ready with session ...
INFO  Search OK! Found 24 tracks. LoadType: search
```

---

## Next Steps

| | |
|---|---|
| [Node Options](./node-options.md) | Configure nodes in detail |
| [Events](./events.md) | All events and how to use them |
| [Searching](./searching.md) | Load and search tracks |
| [Players](./players.md) | Create and control players |
| [Session Resuming](./resuming.md) | Survive restarts gracefully |
| [Filters](./filters.md) | Audio filters and EQ |
