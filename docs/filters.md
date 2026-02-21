# 🎚️ Audio Filters & EQ

Apply real-time audio effects to a player using Lavalink's filter system.

---

## Applying Filters

Filters are sent via `LavalinkPlayOptions.filters` when calling `node.update_player()`:

```rust
use lavalink_client_rs::types::filters::FilterData;
use lavalink_client_rs::types::player::LavalinkPlayOptions;

let opts = LavalinkPlayOptions {
    filters: Some(FilterData {
        volume: Some(1.2), // 20% louder
        ..Default::default()
    }),
    ..Default::default()
};

node.update_player("GUILD_ID", false, &opts).await?;
```

---

## Clearing Filters

Send an empty `FilterData` to reset all filters:

```rust
node.update_player("GUILD_ID", false, &LavalinkPlayOptions {
    filters: Some(FilterData::default()),
    ..Default::default()
}).await?;
```

---

## Filter Reference

### `volume` — Volume Multiplier

```rust
FilterData {
    volume: Some(1.0), // 1.0 = 100% (default), 0.5 = 50%, 2.0 = 200%
    ..Default::default()
}
```
> Range: `0.0` to `5.0`

---

### `equalizer` — 15-Band EQ

```rust
use lavalink_client_rs::types::filters::EQBand;

FilterData {
    equalizer: Some(vec![
        EQBand { band: 0, gain: 0.25 },  // bass boost
        EQBand { band: 1, gain: 0.25 },
        EQBand { band: 2, gain: 0.15 },
    ]),
    ..Default::default()
}
```

| Field | Type | Description |
|---|---|---|
| `band` | `i32` | Band index, `0`–`14` |
| `gain` | `f32` | Gain value, `-0.25` to `1.0` (0.0 = no change) |

> **Bass Boost tip:** Raise bands 0–3, lower bands 10–14.

---

### `timescale` — Speed / Pitch / Rate

```rust
use lavalink_client_rs::types::filters::Timescale;

FilterData {
    timescale: Some(Timescale {
        speed: Some(1.2),  // 1.2x faster
        pitch: Some(1.0),  // no pitch change
        rate: Some(1.0),   // no rate change
    }),
    ..Default::default()
}
```

> **Nightcore effect:** `speed: 1.3, pitch: 1.3`  
> **Vaporwave effect:** `speed: 0.8, pitch: 0.8`

---

### `karaoke` — Vocal Removal

```rust
use lavalink_client_rs::types::filters::Karaoke;

FilterData {
    karaoke: Some(Karaoke {
        level: Some(1.0),
        mono_level: Some(1.0),
        filter_band: Some(220.0),
        filter_width: Some(100.0),
    }),
    ..Default::default()
}
```

---

### `tremolo` — Volume Oscillation (Wobble Effect)

```rust
use lavalink_client_rs::types::filters::Tremolo;

FilterData {
    tremolo: Some(Tremolo {
        frequency: Some(2.0), // oscillations per second
        depth: Some(0.5),     // 0.0–1.0
    }),
    ..Default::default()
}
```

---

### `vibrato` — Pitch Oscillation

```rust
use lavalink_client_rs::types::filters::Vibrato;

FilterData {
    vibrato: Some(Vibrato {
        frequency: Some(2.0), // 0–14 Hz
        depth: Some(0.5),     // 0.0–1.0
    }),
    ..Default::default()
}
```

---

### `rotation` — 8D Audio Effect

```rust
use lavalink_client_rs::types::filters::Rotation;

FilterData {
    rotation: Some(Rotation {
        rotation_hz: Some(0.2), // rotations per second
    }),
    ..Default::default()
}
```

---

### `low_pass` — Low Pass Filter (cut treble)

```rust
use lavalink_client_rs::types::filters::LowPass;

FilterData {
    low_pass: Some(LowPass {
        smoothing: Some(20.0), // higher = more smoothing
    }),
    ..Default::default()
}
```

---

### `distortion` — Distortion Effect

```rust
use lavalink_client_rs::types::filters::Distortion;

FilterData {
    distortion: Some(Distortion {
        sin_offset: Some(0.0),
        sin_scale: Some(1.0),
        cos_offset: Some(0.0),
        cos_scale: Some(1.0),
        tan_offset: Some(0.0),
        tan_scale: Some(1.0),
        offset: Some(0.0),
        scale: Some(1.0),
    }),
    ..Default::default()
}
```

---

### `channel_mix` — Stereo / Mono Channel Mix

```rust
use lavalink_client_rs::types::filters::ChannelMix;

FilterData {
    channel_mix: Some(ChannelMix {
        left_to_left: Some(0.5),
        left_to_right: Some(0.5),
        right_to_left: Some(0.5),
        right_to_right: Some(0.5),
    }),
    ..Default::default()
}
```

> Setting all to `0.5` creates a **mono** mix.

---

## Combining Filters

All filters can be combined in a single `FilterData`:

```rust
FilterData {
    volume: Some(0.9),
    equalizer: Some(vec![
        EQBand { band: 0, gain: 0.3 },
        EQBand { band: 1, gain: 0.2 },
    ]),
    timescale: Some(Timescale {
        speed: Some(1.1),
        pitch: Some(1.1),
        rate: Some(1.0),
    }),
    rotation: Some(Rotation { rotation_hz: Some(0.15) }),
    ..Default::default()
}
```
