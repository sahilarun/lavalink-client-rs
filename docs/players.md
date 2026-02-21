# 🎵 Player Management

How to create, control, and destroy players on Lavalink nodes.

---

## Create a Player

```rust
use lavalink_client_rs::types::player::PlayerOptions;

let player = manager.create_player(PlayerOptions {
    guild_id: "GUILD_ID".to_string(),
    ..Default::default()
}).await?;
```

This assigns the player to the least-used node automatically.

---

## Play a Track

After [searching](./searching.md) for a track, use its `encoded` field to play it:

```rust
use lavalink_client_rs::types::player::LavalinkPlayOptions;

let node = manager.node_manager.get_node("MainNode").unwrap();

let opts = LavalinkPlayOptions {
    encoded_track: track.encoded.clone(), // from SearchResult
    ..Default::default()
};

node.update_player("GUILD_ID", false, &opts).await?;
// second arg `no_replace`: true = don't replace if something is already playing
```

---

## Pause / Resume

```rust
// Pause
node.update_player("GUILD_ID", false, &LavalinkPlayOptions {
    paused: Some(true),
    ..Default::default()
}).await?;

// Resume
node.update_player("GUILD_ID", false, &LavalinkPlayOptions {
    paused: Some(false),
    ..Default::default()
}).await?;
```

---

## Seek

```rust
node.update_player("GUILD_ID", false, &LavalinkPlayOptions {
    position: Some(60_000), // seek to 60 seconds (in ms)
    ..Default::default()
}).await?;
```

---

## Volume

```rust
// Volume is 0–1000, where 100 is the default (100%)
node.update_player("GUILD_ID", false, &LavalinkPlayOptions {
    volume: Some(75),
    ..Default::default()
}).await?;
```

---

## Stop Playback

To stop the current track without destroying the player:

```rust
node.update_player("GUILD_ID", false, &LavalinkPlayOptions {
    encoded_track: None, // clears the current track
    ..Default::default()
}).await?;
```

---

## Destroy a Player

```rust
// Destroy on the Lavalink node side
node.destroy_player("GUILD_ID").await?;

// Remove from manager's local state
manager.delete_player("GUILD_ID").await;
```

---

## Fetch Player State

```rust
let player_state = node.fetch_player("GUILD_ID").await?;
println!("Volume: {}", player_state.volume);
println!("Paused: {}", player_state.paused);
println!("Connected: {}", player_state.state.connected);
println!("Position: {}ms", player_state.state.position);
```

---

## Voice Server Update

When Discord sends a `VOICE_SERVER_UPDATE` gateway event, forward it:

```rust
manager.voice_server_update("GUILD_ID", endpoint, session_id, token).await?;
```
