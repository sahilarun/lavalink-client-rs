# 📢 Events

All events arrive via the `mpsc::Receiver<LavalinkMessage>` returned from `LavalinkManager::new()`.

---

## Receiving Events

```rust
let (manager, mut rx) = LavalinkManager::new(options);

tokio::spawn(async move {
    while let Some(event) = rx.recv().await {
        match event {
            LavalinkMessage::Ready { session_id, resumed } => { /* ... */ }
            LavalinkMessage::Stats(stats)                  => { /* ... */ }
            LavalinkMessage::PlayerUpdate { guild_id, state } => { /* ... */ }
            LavalinkMessage::Event(player_event)           => { /* ... */ }
        }
    }
});
```

---

## Event Reference

### `LavalinkMessage::Ready`

Fired when a node WebSocket connects and Lavalink signals it's ready.

```rust
LavalinkMessage::Ready { session_id, resumed } => {
    // session_id: String — save this for session resuming
    // resumed: bool — true if a previous session was resumed
    info!("Node ready! Session: {}", session_id);
}
```

---

### `LavalinkMessage::Stats`

Fired periodically (~every 60s) with node health information.

```rust
LavalinkMessage::Stats(stats) => {
    println!("Players: {}/{}", stats.playing_players, stats.players);
    println!("CPU: {:.2}% lavalink load", stats.cpu.lavalink_load * 100.0);
    println!("Memory used: {} bytes", stats.memory.used);
    println!("Uptime: {}ms", stats.uptime);
}
```

Fields available on `NodeStats`:

| Field | Type | Description |
|---|---|---|
| `players` | `i32` | Total number of players |
| `playing_players` | `i32` | Currently playing players |
| `uptime` | `i64` | Node uptime in milliseconds |
| `memory.used` | `u64` | Memory used (bytes) |
| `memory.free` | `u64` | Memory free (bytes) |
| `cpu.cores` | `i32` | CPU core count |
| `cpu.lavalink_load` | `f32` | Lavalink CPU load (0.0–1.0) |
| `cpu.system_load` | `f32` | System CPU load (0.0–1.0) |

---

### `LavalinkMessage::PlayerUpdate`

Fired every ~5s for each active player with position info.

```rust
LavalinkMessage::PlayerUpdate { guild_id, state } => {
    println!("Guild {}: position={}ms connected={}", guild_id, state.position, state.connected);
}
```

Fields on `LavalinkPlayerState`:

| Field | Type | Description |
|---|---|---|
| `time` | `i64` | Unix timestamp (ms) of the update |
| `position` | `i64` | Current track position in ms |
| `connected` | `bool` | Whether the player is connected to voice |
| `ping` | `i64` | Voice server ping in ms |

---

### `LavalinkMessage::Event(PlayerEvent)`

Player events triggered by track lifecycle changes.

```rust
LavalinkMessage::Event(player_event) => {
    match player_event {
        PlayerEvent::TrackStartEvent { guild_id, track } => {
            println!("▶ Now playing in {}: {}", guild_id, track.info.title);
        }
        PlayerEvent::TrackEndEvent { guild_id, track, reason } => {
            // reason: "finished" | "loadFailed" | "stopped" | "replaced" | "cleanup"
            println!("⏹ Track ended in {} ({}): {}", guild_id, reason, track.info.title);
        }
        PlayerEvent::TrackExceptionEvent { guild_id, exception, track } => {
            println!("❌ Error in {}: {:?} — {}", guild_id, exception, track.info.title);
        }
        PlayerEvent::TrackStuckEvent { guild_id, track, threshold_ms } => {
            println!("⚠ Track stuck in {} after {}ms: {}", guild_id, threshold_ms, track.info.title);
        }
        PlayerEvent::WebSocketClosedEvent { guild_id, code, reason, by_remote } => {
            println!("🔌 WS closed in {} — code={} reason={} by_remote={}", guild_id, code, reason, by_remote);
        }
    }
}
```

#### TrackEnd Reasons

| Reason | Description |
|---|---|
| `"finished"` | Track played to completion |
| `"loadFailed"` | Track failed to load/stream |
| `"stopped"` | Stopped manually |
| `"replaced"` | A new track was played |
| `"cleanup"` | Player was cleaned up |
