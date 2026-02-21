# 🔒 Session Resuming

Session resuming allows your bot to reconnect to Lavalink after a restart without losing active players.

---

## How It Works

1. When a node connects, Lavalink sends a `Ready` event with a `session_id`
2. Save that `session_id` somewhere persistent (file, database, etc.)
3. On restart, pass the saved `session_id` in `LavalinkNodeOptions`
4. Lavalink will resume your previous session — players keep playing

---

## Step 1: Save the Session ID

```rust
use lavalink_client_rs::types::events::LavalinkMessage;

while let Some(event) = rx.recv().await {
    match event {
        LavalinkMessage::Ready { session_id, resumed } => {
            if !resumed {
                // New session — save it for future restarts
                std::fs::write("lavalink_session.txt", &session_id)
                    .expect("Failed to save session ID");
            }
            tracing::info!("Session: {} (resumed={})", session_id, resumed);
        }
        _ => {}
    }
}
```

---

## Step 2: Load the Session ID on Restart

```rust
use lavalink_client_rs::node::LavalinkNodeOptions;

let saved_session = std::fs::read_to_string("lavalink_session.txt").ok();

let node_options = LavalinkNodeOptions {
    id: "MainNode".to_string(),
    host: "localhost".to_string(),
    port: 2333,
    authorization: "youshallnotpass".to_string(),
    secure: Some(false),
    request_timeout: Some(10_000),
    session_id: saved_session, // ← this enables resuming
};

manager.node_manager.add_node(node_options, user_id, client_name).await?;
```

---

## Step 3: Verify in the Ready Event

```rust
LavalinkMessage::Ready { session_id, resumed } => {
    if resumed {
        tracing::info!("✅ Session resumed successfully: {}", session_id);
        // Your players are still active on the node
    } else {
        tracing::warn!("⚠ New session started (could not resume): {}", session_id);
        // Save new session
        std::fs::write("lavalink_session.txt", &session_id).ok();
    }
}
```

---

## Enable Resuming on the Node Side

Lavalink needs to be told to hold the session open while you reconnect. Call `update_session` right after connecting:

```rust
LavalinkMessage::Ready { session_id, .. } => {
    if let Some(node) = manager.node_manager.get_node("MainNode") {
        // Tell Lavalink to keep the session alive for 60 seconds if we disconnect
        node.update_session(Some(true), Some(60)).await.ok();
    }
}
```

This sets `resuming=true` and a 60-second timeout window via `PATCH /v4/sessions/{sessionId}`.

---

## Notes

- Session IDs are node-specific — save one per node if you have multiple
- If the node restarts, the session is lost regardless
- Lavalink v4 defaults to **not** resuming sessions unless you explicitly call `update_session`
