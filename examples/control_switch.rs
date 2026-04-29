use rustplus::{RustEntityType, RustSocket};

#[tokio::main]
async fn main() -> Result<(), rustplus::Error> {
    let mut socket = RustSocket::builder()
        .ip("YOUR_SERVER_IP")
        .port(28082)
        .player_id(76561199000000000)
        .player_token(123456)
        .build()?;

    socket.connect().await?;

    // Replace with your actual paired entity_id
    let entity_id: u32 = 12345;

    let info = socket.get_entity_info(entity_id).await?;
    println!("Entity type: {:?}", info.entity_type);

    if info.entity_type != RustEntityType::Switch {
        println!("This entity is not a switch!");
        return Ok(());
    }

    let current_state = info.payload.as_ref().map(|p| p.value).unwrap_or(false);
    println!(
        "Current state: {}",
        if current_state { "ON" } else { "OFF" }
    );

    // Toggle
    socket.set_entity_value(entity_id, !current_state).await?;
    println!("Toggled to: {}", if !current_state { "ON" } else { "OFF" });

    socket.disconnect().await?;
    Ok(())
}
