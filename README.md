# rustplus

[![Crates.io](https://img.shields.io/crates/v/rustplus.svg)](https://crates.io/crates/rustplus)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

Async Rust client for the [Rust+ Game API](https://rust.facepunch.com/companion). 
Connect to Rust game servers, control smart devices, monitor team chat, and 
receive real-time events through the official companion app protocol.

> ⚠️ **Unofficial library.** Not affiliated with Facepunch Studios.

## Features

- 🔌 WebSocket connection with automatic reconnect-friendly design
- 📦 Type-safe wrappers around protobuf messages
- 🎯 Typed event system (chat, team changes, entity updates)
- ⏱️ Built-in dual-tier rate limiter (server + socket level)
- 🔁 Automatic retry on `rate_limit` errors
- 🚀 Builder pattern for clean configuration
- ⚡ Async/await with Tokio

## Installation

```toml
[dependencies]
rustplus = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use rustplus::RustSocket;

#[tokio::main]
async fn main() -> Result<(), rustplus::Error> {
    let mut socket = RustSocket::builder()
        .ip("YOUR_SERVER_IP")              // String
        .port(28082)                       // u16
        .player_id(76561199000000000)      // u64
        .player_token(123456)              // i32
        .build()?;

    socket.connect().await?;

    // Get server info
    let info = socket.get_info().await?;
    println!("Server: {} ({}/{})", info.name, info.players, info.max_players);

    // Send a team chat message
    socket.send_team_message("Hello from Rust!".to_string()).await?;

    // Listen for events
    let mut events = socket.events();
    while let Some(event) = events.next().await {
        match event {
            rustplus::RustEvent::TeamMessage(msg) => {
                println!("[CHAT] {}: {}", msg.name, msg.message);
            }
            rustplus::RustEvent::EntityChanged { entity_id, payload } => {
                println!("Entity {} -> value={}", entity_id, payload.value);
            }
            _ => {}
        }
    }

    Ok(())
}
```

## Getting Credentials

To use this library, you will need a `player_id` (Steam ID) and a `player_token`.
You can obtain these by pairing with the Rust server via an extension created by the brilliant Ollie. See the [pairing guide](https://rplus.ollieee.xyz/getting-started/getting-player-details).

## Examples

See the [`examples/`](examples/) directory for runnable examples:

- `ws_connection.rs` — basic connection and `get_info`
- `try_spam.rs` — rate limiter demonstration (70 sequential requests)
- `listen_events.rs` — listen for real-time team/entity events
- `control_switch.rs` — toggle a Smart Switch device

Run with:
```bash
cargo run --example ws_connection
```

## Credits

Inspired by the Python [rustplus](https://github.com/olijeffers0n/rustplus) 
library by olijeffers0n. Protocol definitions taken from the official 
companion app.

## Contributing

Contributions welcome! Please open an issue or PR. By contributing you agree 
that your contributions will be licensed under the AGPL-3.0.

## License

This project is licensed under the [GNU AGPL v3.0](LICENSE) — see the LICENSE 
file for details.