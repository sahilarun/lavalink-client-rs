# 📚 lavalink-client-rs — Documentation

> Complete guide for using `lavalink-client-rs`, a Rust port of [`lavalink-client`](https://github.com/Tomato6966/lavalink-client) by Tomato6966.

> [!NOTE]
> This library was originally written for **[Tranquil](https://github.com/sahilarun/tranquil)** — a custom Lavalink-compatible audio server. It works well with standard **Lavalink v4** nodes too.

---

## Table of Contents

1. [Getting Started](#-getting-started)
2. [LavalinkManager Options](#-lavalinkmanager-options)
3. [Node Options](#-node-options)
4. [Events Reference](#-events-reference)
5. [Searching & Loading Tracks](#-searching--loading-tracks)
6. [Player Management](#-player-management)
7. [Session Resuming](#-session-resuming)
8. [Audio Filters](#-audio-filters)
9. [Node REST API Methods](#-node-rest-api-methods)

---

## 🚀 Getting Started

### 1. Add the dependency

```toml
# Cargo.toml
[dependencies]
lavalink-client-rs = { git = "https://github.com/sahilarun/lavalink-client-rs" }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

### 2. Create the manager and add a node

```rust
use lavalink_client_rs::manager::{LavalinkManager, LavalinkManagerOptions};
use lavalink_client_rs::node::LavalinkNodeOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let options = LavalinkManagerOptions {
        user_id: "YOUR_BOT_USER_ID".to_string(),
        client_name: "MyBot/1.0".to_string(),
        auto_skip: true,
        ..Default::default()
    };

    let (manager, mut rx) = LavalinkManager::new(options);

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
        "YOUR_BOT_USER_ID".to_string(),
        "MyBot/1.0".to_string(),
    ).await?;

    // Event loop
    while let Some(event) = rx.recv().await {
        println!("Event: {:?}", event);
    }

    Ok(())
}
```

---

## ⚙️ LavalinkManager Options

| Field | Type | Default | Description |
|---|---|---|---|
| `user_id` | `String` | `""` | Your Discord bot's user ID (sent in WebSocket headers) |
| `client_name` | `String` | `"lavalink-client-rs"` | Sent as `Client-Name` header to Lavalink |
| `auto_skip` | `bool` | `true` | Automatically play next track on `TrackEndEvent` |
| `send_to_shard` | `bool` | `false` | Whether to route voice updates per shard |
| `emit_new_songs_only` | `bool` | `false` | Only emit `trackStart` for newly added songs |

```rust
let options = LavalinkManagerOptions {
    user_id: "123456789".to_string(),
    client_name: "MyBot/2.0".to_string(),
    auto_skip: true,
    send_to_shard: false,
    emit_new_songs_only: false,
};
```

---

## 🔌 Node Options

| Field | Type | Description |
|---|---|---|
| `id` | `String` | Unique name for this node |
| `host` | `String` | Hostname of the Lavalink server |
| `port` | `u16` | Port number (usually 2333) |
| `authorization` | `String` | Password for the Lavalink server |
| `secure` | `Option<bool>` | `true` = use `wss://` / `https://`, `false` = `ws://` / `http://` |
| `request_timeout` | `Option<u64>` | REST request timeout in milliseconds |
| `session_id` | `Option<String>` | Previous session ID for [session resuming](#-session-resuming) |

### Multiple Nodes

```rust
// Add a primary node
manager.node_manager.add_node(primary_options, user_id.clone(), client_name.clone()).await?;

// Add a fallback node
manager.node_manager.add_node(fallback_options, user_id, client_name).await?;

// Get a specific node
if let Some(node) = manager.node_manager.get_node("MainNode") {
    // use the node
}

// Get the least-loaded node (auto load balancing)
if let Some(node) = manager.node_manager.least_used_node() {
    // use the node
}
```

---

## 📢 Events Reference

All events come through the `mpsc::Receiver<LavalinkMessage>` returned from `LavalinkManager::new()`.

```rust
let (manager, mut rx) = LavalinkManager::new(options);

tokio::spawn(async move {
    while let Some(event) = rx.recv().await {
        match event {
            LavalinkMessage::Ready { session_id, resumed } => {
                println!("Connected! Session: {} (resumed: {})", session_id, resumed);
            }
            LavalinkMessage::Stats(stats) => {
                println!("Players: {}, CPU: {:.2}", stats.players, stats.cpu.lavalink_load);
            }
            LavalinkMessage::PlayerUpdate { guild_id, state } => {
                println!("Guild {} position: {}ms", guild_id, state.position);
            }
            LavalinkMessage::Event(player_event) => {
                match player_event {
                    PlayerEvent::TrackStartEvent { guild_id, track } => {
                        println!("Now playing in {}: {}", guild_id, track.info.title);
                    }
                    PlayerEvent::TrackEndEvent { guild_id, track, reason } => {
                        println!("Track ended in {} ({}): {}", guild_id, reason, track.info.title);
                    }
                    PlayerEvent::TrackExceptionEvent { guild_id, exception, .. } => {
                        println!("Error in {}: {:?}", guild_id, exception);
                    }
                    PlayerEvent::TrackStuckEvent { guild_id, threshold_ms, .. } => {
                        println!("Track stuck in {} after {}ms", guild_id, threshold_ms);
                    }
                    PlayerEvent::WebSocketClosedEvent { guild_id, code, reason, .. } => {
                        println!("WS closed in {} ({}: {})", guild_id, code, reason);
                    }
                }
            }
        }
    }
});
```

### Event Table

| Event | When |
|---|---|
| `Ready` | Node WebSocket connected and ready |
| `Stats` | Every ~60s: CPU, memory, players info |
| `PlayerUpdate` | Every ~5s: player position and state |
| `Event(TrackStartEvent)` | A track begins playing |
| `Event(TrackEndEvent)` | A track finishes (or is skipped) |
| `Event(TrackExceptionEvent)` | Error while playing a track |
| `Event(TrackStuckEvent)` | Track is stuck and not progressing |
| `Event(WebSocketClosedEvent)` | Discord voice WebSocket closed for a guild |

---

## 🔍 Searching & Loading Tracks

Use `node.search(query)` to search or load tracks. This wraps Lavalink's `/v4/loadtracks` endpoint.

### Search Prefixes

| Prefix | Source |
|---|---|
| `ytsearch:` | YouTube |
| `ytmsearch:` | YouTube Music |
| `scsearch:` | SoundCloud |
| `dzsearch:` | Deezer |
| `spsearch:` | Spotify |
| `https://...` | Direct URL (YouTube, SoundCloud, etc.) |

### Handling All Load Types

```rust
use lavalink_client_rs::types::events::SearchResult;

let result: SearchResult = node.search("ytsearch:bohemian rhapsody").await?;

match result.load_type.as_str() {
    "search" => {
        // Multiple tracks returned from a search query
        let tracks = result.tracks();
        println!("Found {} results", tracks.len());
        for track in &tracks {
            println!("  - {} by {} ({}ms)", track.info.title, track.info.author, track.info.length);
        }
    }
    "track" => {
        // A single direct track URL was resolved
        let tracks = result.tracks();
        if let Some(track) = tracks.first() {
            println!("Loaded: {}", track.info.title);
        }
    }
    "playlist" => {
        // A playlist URL was loaded
        let tracks = result.tracks();
        if let Some(playlist) = result.playlist() {
            println!("Playlist '{}' with {} tracks", playlist.name.unwrap_or_default(), tracks.len());
        }
    }
    "error" => {
        if let Some(err) = result.error() {
            println!("Load error: {:?} — {:?}", err.severity, err.message);
        }
    }
    "empty" => {
        println!("No results found for your query.");
    }
    _ => {}
}
```

---

## 🎵 Player Management

### Create a Player

```rust
use lavalink_client_rs::types::player::PlayerOptions;

let player = manager.create_player(PlayerOptions {
    guild_id: "GUILD_ID".to_string(),
    ..Default::default()
}).await?;
```

### Play a Track

```rust
use lavalink_client_rs::types::player::LavalinkPlayOptions;

let play_opts = LavalinkPlayOptions {
    encoded_track: Some(track.encoded.clone().unwrap()),
    ..Default::default()
};

node.update_player("GUILD_ID", false, &play_opts).await?;
```

### Pause / Resume

```rust
let pause_opts = LavalinkPlayOptions {
    paused: Some(true),
    ..Default::default()
};
node.update_player("GUILD_ID", false, &pause_opts).await?;
```

### Set Volume

```rust
let vol_opts = LavalinkPlayOptions {
    volume: Some(80), // 0–1000 (100 = default)
    ..Default::default()
};
node.update_player("GUILD_ID", false, &vol_opts).await?;
```

### Seek

```rust
let seek_opts = LavalinkPlayOptions {
    position: Some(30_000), // seek to 30 seconds (in ms)
    ..Default::default()
};
node.update_player("GUILD_ID", false, &seek_opts).await?;
```

### Destroy a Player

```rust
node.destroy_player("GUILD_ID").await?;
manager.delete_player("GUILD_ID").await;
```

---

## 🔒 Session Resuming

To resume a previous session after a restart, save the `session_id` from the `Ready` event and pass it in `LavalinkNodeOptions`:

```rust
// On first start — save the session_id somewhere (file, DB, etc.)
LavalinkMessage::Ready { session_id, .. } => {
    std::fs::write("session.txt", &session_id).ok();
}

// On restart — load and pass it
let saved_session = std::fs::read_to_string("session.txt").ok();

let node_options = LavalinkNodeOptions {
    id: "MainNode".to_string(),
    host: "localhost".to_string(),
    port: 2333,
    authorization: "youshallnotpass".to_string(),
    secure: Some(false),
    request_timeout: Some(10_000),
    session_id: saved_session, // ← resumes the session
};
```

When `resumed: true` comes back in the `Ready` event, your previous players are still active on the node.

---

## 🎚️ Audio Filters

Apply audio filters to a player via `LavalinkPlayOptions.filters`:

```rust
use lavalink_client_rs::types::filters::{FilterData, EQBand};

let filters = FilterData {
    volume: Some(1.0),
    equalizer: Some(vec![
        EQBand { band: 0, gain: 0.25 },
        EQBand { band: 1, gain: 0.25 },
    ]),
    timescale: Some(lavalink_client_rs::types::filters::Timescale {
        speed: Some(1.1),
        pitch: Some(1.0),
        rate: Some(1.0),
    }),
    ..Default::default()
};

let opts = LavalinkPlayOptions {
    filters: Some(filters),
    ..Default::default()
};
node.update_player("GUILD_ID", false, &opts).await?;
```

### Available Filters

| Filter | Description |
|---|---|
| `volume` | Overall volume multiplier (0.0–5.0) |
| `equalizer` | 15-band EQ (bands 0–14, gain -0.25 to 1.0) |
| `karaoke` | Removes vocals from audio |
| `timescale` | Change speed, pitch, and rate |
| `tremolo` | Rapid volume oscillation |
| `vibrato` | Rapid pitch oscillation |
| `rotation` | 8D audio effect |
| `distortion` | Audio distortion |
| `channel_mix` | Mix left/right channels |
| `low_pass` | Cut high frequencies |

---

## 🌐 Node REST API Methods

All methods are on `Arc<LavalinkNode>`:

```rust
let node = manager.node_manager.get_node("MainNode").unwrap();

// Check server info
let info = node.fetch_info().await?;
let version = node.fetch_version().await?;

// Players
let players = node.fetch_all_players().await?;
let player = node.fetch_player("GUILD_ID").await?;

// Decode track(s)
let track = node.decode_single_track("BASE64_ENCODED_TRACK").await?;
let tracks = node.decode_multiple_tracks(vec!["track1".into(), "track2".into()]).await?;

// Session
let session = node.update_session(Some(true), Some(60)).await?; // enable resuming, 60s timeout

// Route planner
let status = node.route_planner_status().await?;
node.route_planner_unmark_failed_address("1.2.3.4").await?;

// Lyrics (if supported by your node/plugin)
node.subscribe_lyrics("GUILD_ID").await?;
let lyrics = node.get_current_lyrics("GUILD_ID", false).await?;

// Raw requests
node.request(reqwest::Method::GET, "/v4/info").await?;
```

---

## 🔗 Related Projects

| Project | Description |
|---|---|
| [Tranquil](https://github.com/sahilarun/tranquil) | The Lavalink-compatible server this client was built for |
| [lavalink-client (TS)](https://github.com/Tomato6966/lavalink-client) | Original TypeScript library this is ported from |
| [Lavalink](https://github.com/lavalink-devs/Lavalink) | Official Lavalink audio server |
