# 🔌 Node Options

Full reference for `LavalinkNodeOptions`.

---

## Fields

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | `String` | ✅ | Unique name for this node (used to look it up later) |
| `host` | `String` | ✅ | Hostname/IP of the Lavalink server |
| `port` | `u16` | ✅ | Port number (default Lavalink: `2333`) |
| `authorization` | `String` | ✅ | Server password (`lavalink.server.password` in `application.yml`) |
| `secure` | `Option<bool>` | ❌ | `true` = use `wss://` + `https://`, `false` = `ws://` + `http://` |
| `request_timeout` | `Option<u64>` | ❌ | REST request timeout in ms (default: no timeout) |
| `session_id` | `Option<String>` | ❌ | Previous session ID for resuming. See [Session Resuming](./resuming.md) |

---

## Example

```rust
use lavalink_client_rs::node::LavalinkNodeOptions;

let node_options = LavalinkNodeOptions {
    id: "MainNode".to_string(),
    host: "localhost".to_string(),
    port: 2333,
    authorization: "youshallnotpass".to_string(),
    secure: Some(false),
    request_timeout: Some(10_000), // 10 seconds
    session_id: None,
};
```

---

## Multiple Nodes

You can register multiple nodes. The manager will use `least_used_node()` for auto load-balancing:

```rust
// Primary node
manager.node_manager.add_node(primary_opts, user_id.clone(), client_name.clone()).await?;

// Fallback node
manager.node_manager.add_node(fallback_opts, user_id.clone(), client_name.clone()).await?;

// Get a specific node by ID
let node = manager.node_manager.get_node("MainNode");

// Get least-used node (for player assignment)
let node = manager.node_manager.least_used_node();

// Remove a node
manager.node_manager.remove_node("MainNode");
```

---

## Secure (TLS) Nodes

```rust
LavalinkNodeOptions {
    host: "lavalink.mybot.com".to_string(),
    port: 443,
    secure: Some(true), // uses wss:// and https://
    ..
}
```

---

## Node REST API

Once a node is connected, you have access to all Lavalink v4 REST endpoints. See [GUIDE.md](./GUIDE.md#-node-rest-api-methods) for the full list.
