use rustplus::{RustEvent, RustSocket};

#[tokio::main]
async fn main() -> Result<(), rustplus::Error> {
    let mut socket = RustSocket::builder()
        .ip("YOUR_SERVER_IP")
        .port(28082)
        .player_id(76561199000000000)
        .player_token(123456)
        .build()?;

    socket.connect().await?;
    println!("Connected. Listening for events... (Ctrl+C to exit)");

    let mut events = socket.events();
    while let Some(event) = events.next().await {
        match event {
            RustEvent::TeamMessage(msg) => {
                println!("[TEAM CHAT] {}: {}", msg.name, msg.message);
            }
            RustEvent::TeamChanged {
                player_id,
                team_info,
            } => {
                println!(
                    "[TEAM CHANGED] player {} caused update, leader: {}",
                    player_id, team_info.leader_steam_id
                );
            }
            RustEvent::EntityChanged { entity_id, payload } => {
                println!("[ENTITY {}] value={}", entity_id, payload.value);
            }
            RustEvent::ClanMessage { clan_id, message } => {
                println!(
                    "[CLAN {} CHAT] {}: {}",
                    clan_id, message.name, message.message
                );
            }
            _ => {}
        }
    }

    Ok(())
}
