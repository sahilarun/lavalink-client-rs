# 🔍 Searching & Loading Tracks

How to use `node.search()` to find and load tracks.

---

## Basic Search

```rust
let result = node.search("ytsearch:bohemian rhapsody").await?;
```

This calls Lavalink's `/v4/loadtracks` endpoint. The response has a `load_type` field and a polymorphic `data` field.

---

## Search Prefixes

| Prefix | Source |
|---|---|
| `ytsearch:` | YouTube |
| `ytmsearch:` | YouTube Music |
| `scsearch:` | SoundCloud |
| `dzsearch:` | Deezer |
| `spsearch:` | Spotify |
| `jssearch:` | JioSaavn |
| `https://...` | Direct URL (auto-detected) |

---

## Handling All Load Types

```rust
use lavalink_client_rs::types::events::SearchResult;

let result: SearchResult = node.search("ytsearch:hello adele").await?;

match result.load_type.as_str() {
    "search" => {
        // Multiple results from a keyword search
        let tracks = result.tracks();
        println!("Found {} results:", tracks.len());
        for track in &tracks {
            println!("  {} — {} ({}ms)", track.info.title, track.info.author, track.info.length);
        }
    }
    "track" => {
        // A single track from a direct URL
        let tracks = result.tracks();
        if let Some(track) = tracks.first() {
            println!("Loaded: {} [{}]", track.info.title, track.encoded.as_deref().unwrap_or("?"));
        }
    }
    "playlist" => {
        // A playlist URL
        let tracks = result.tracks();
        if let Some(playlist) = result.playlist() {
            println!("Playlist '{}': {} tracks", playlist.name.unwrap_or_default(), tracks.len());
        }
    }
    "error" => {
        if let Some(err) = result.error() {
            println!("Load error [{}]: {:?}", err.severity, err.message);
        }
    }
    "empty" => {
        println!("No results found.");
    }
    _ => {}
}
```

---

## SearchResult API

`SearchResult` has three convenience methods:

| Method | Returns | When to use |
|---|---|---|
| `result.tracks()` | `Vec<LavalinkTrack>` | `"search"`, `"track"`, or `"playlist"` |
| `result.playlist()` | `Option<PlaylistInfoData>` | `"playlist"` only |
| `result.error()` | `Option<Exception>` | `"error"` only |

---

## Track Fields

```rust
track.encoded           // Option<String> — base64 encoded track (used for playback)
track.info.title        // String
track.info.author       // String
track.info.length       // i64 — duration in milliseconds
track.info.identifier   // String — platform-specific ID
track.info.uri          // Option<String> — track URL
track.info.artwork_url  // Option<String>
track.info.is_stream    // bool
track.info.is_seekable  // bool
track.info.source_name  // SourceNames enum (Youtube, Soundcloud, etc.)
track.info.isrc         // Option<String>
```
