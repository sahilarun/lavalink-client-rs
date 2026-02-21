# lavalink-client-rs

> **A Rust port of the popular [`lavalink-client`](https://github.com/Tomato6966/lavalink-client) TypeScript library.**  
> A flexible, async Lavalink v4 client for building Discord music bots in Rust.

> [!NOTE]
> This library works well with standard **Lavalink v4** nodes, but was originally built and tested against **[Tranquil](https://github.com/sahilarun/tranquil)** — a custom Lavalink-compatible audio server. If you're using Tranquil, this is the go-to client for Rust.

---

## 🚀 Features

- ✅ **Lavalink v4 Native** — Full support for Lavalink v4's WebSocket protocol and REST API
- ✨ **Async/Await** — Built on `tokio` for non-blocking, high-performance I/O
- 🎵 **Track Search** — Search and load tracks via `/v4/loadtracks` with polymorphic response handling
- 🎚️ **Filters & EQ** — Built-in types for all Lavalink audio filters
- 🔌 **Node Manager** — Manage multiple Lavalink nodes, auto-select least-used node
- 📢 **Event Bus** — `mpsc` channel-based event system for `Ready`, `Stats`, `PlayerUpdate`, and player events
- 🔒 **Session Resuming** — Pass a `session_id` in node options to resume a previous session
- 🧑‍💻 **Developer Friendly** — Clean, idiomatic Rust API mirroring the original TypeScript library's concepts
- 💪 **Player Management** — Create, update, and destroy players on Lavalink nodes via REST

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
lavalink-client-rs = { git = "https://github.com/sahilarun/lavalink-client-rs" }
```

Or clone and use as a local path dependency:

```toml
[dependencies]
lavalink-client-rs = { path = "../lavalink-client-rs" }
```

---

## ⚡ Quick Start

```rust
use lavalink_client_rs::manager::{LavalinkManager, LavalinkManagerOptions};
use lavalink_client_rs::node::LavalinkNodeOptions;
use lavalink_client_rs::types::events::LavalinkMessage;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create the manager
    let options = LavalinkManagerOptions {
        client_name: "MyRustBot".to_string(),
        user_id: "YOUR_BOT_USER_ID".to_string(),
        ..Default::default()
    };

    let (manager, mut rx) = LavalinkManager::new(options);

    // Add a Lavalink node
    let node_options = LavalinkNodeOptions {
        id: "MainNode".to_string(),
        host: "localhost".to_string(),
        port: 2333,
        authorization: "youshallnotpass".to_string(),
        secure: Some(false),
        request_timeout: Some(10000),
        session_id: None, // set to Some("...") to resume a session
    };

    manager.node_manager
        .add_node(node_options, "YOUR_BOT_USER_ID".to_string(), "MyRustBot".to_string())
        .await?;

    let search_manager = manager;

    // Handle events
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                LavalinkMessage::Ready { session_id, .. } => {
                    info!("Node ready! Session: {}", session_id);

                    if let Some(node) = search_manager.node_manager.get_node("MainNode") {
                        match node.search("ytsearch:never gonna give you up").await {
                            Ok(res) => {
                                let tracks = res.tracks();
                                info!("Found {} tracks (loadType: {})", tracks.len(), res.load_type);
                                if let Some(track) = tracks.first() {
                                    info!("First result: {}", track.info.title);
                                }
                            }
                            Err(e) => error!("Search failed: {}", e),
                        }
                    }
                }
                _ => {}
            }
        }
    });

    tokio::signal::ctrl_c().await?;
    Ok(())
}
```

Run the bundled example:

```bash
cargo run --example basic
```

---

## 📖 Documentation

See the **[`docs/`](./docs/)** folder for detailed guides:

| Guide | Description |
|---|---|
| [Getting Started](./docs/getting-started.md) | Full setup walkthrough |
| [Node Options](./docs/node-options.md) | All `LavalinkNodeOptions` fields explained |
| [Searching & Loading Tracks](./docs/searching.md) | How to search, load playlists, and handle all `loadType` values |
| [Events](./docs/events.md) | All `LavalinkMessage` events and how to handle them |
| [Player Management](./docs/players.md) | Creating, updating, and destroying players |
| [Session Resuming](./docs/resuming.md) | How to resume sessions after a restart |
| [Filters](./docs/filters.md) | Using audio filters and EQ |

---

## 📢 Events

Events are received via the `mpsc::Receiver<LavalinkMessage>` returned by `LavalinkManager::new()`.

| Event | Description |
|---|---|
| `LavalinkMessage::Ready { session_id, resumed }` | Node connected and ready |
| `LavalinkMessage::Stats(NodeStats)` | Periodic node stats update |
| `LavalinkMessage::PlayerUpdate { guild_id, state }` | Player position/state update |
| `LavalinkMessage::Event(PlayerEvent)` | Track start/end/error/stuck, WebSocket close |

### Player Events (`LavalinkMessage::Event`)

| Variant | Fields |
|---|---|
| `TrackStartEvent` | `guild_id`, `track` |
| `TrackEndEvent` | `guild_id`, `track`, `reason` |
| `TrackExceptionEvent` | `guild_id`, `track`, `exception` |
| `TrackStuckEvent` | `guild_id`, `track`, `threshold_ms` |
| `WebSocketClosedEvent` | `guild_id`, `code`, `reason`, `by_remote` |

---

## 🛠️ Node Options

```rust
LavalinkNodeOptions {
    id: "MyNode".to_string(),        // Unique identifier for this node
    host: "localhost".to_string(),   // Lavalink host
    port: 2333,                      // Lavalink port
    authorization: "pass".to_string(), // Lavalink password
    secure: Some(false),             // Use wss/https if true
    request_timeout: Some(10000),   // REST request timeout in ms
    session_id: None,                // Previous session ID for resuming
}
```

---

## 🔍 Track Search (`SearchResult`)

The `node.search(query)` method returns a `SearchResult` matching the **Lavalink v4** response format:

```rust
let result = node.search("ytsearch:hello").await?;

match result.load_type.as_str() {
    "search" | "track" => {
        for track in result.tracks() {
            println!("{} by {}", track.info.title, track.info.author);
        }
    }
    "playlist" => {
        if let Some(playlist) = result.playlist() {
            println!("Playlist: {:?}", playlist.name);
        }
    }
    "error" => {
        if let Some(err) = result.error() {
            println!("Error: {:?}", err.message);
        }
    }
    "empty" => println!("No results found."),
    _ => {}
}
```

---

## 🔗 Related Projects

- **[Tranquil](https://github.com/sahilarun/tranquil)** — The custom Lavalink-compatible audio server this client was built for
- **[lavalink-client (TypeScript)](https://github.com/Tomato6966/lavalink-client)** — The original TypeScript library this is ported from
- **[Lavalink](https://github.com/lavalink-devs/Lavalink)** — The audio sending node server

---

## 📄 License

MIT
