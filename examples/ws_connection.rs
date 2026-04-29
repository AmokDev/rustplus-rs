use rustplus::RustSocket;

#[tokio::main]
async fn main() -> Result<(), rustplus::Error> {
    let mut socket = RustSocket::builder()
        .ip("YOUR_SERVER_IP")
        .port(28082)
        .player_id(76561199000000000)
        .player_token(123456)
        .build()?;

    socket.connect().await?;

    let info = socket.get_info().await?;
    println!("Server: {}", info.name);
    println!("Players: {}/{}", info.players, info.max_players);
    println!("Map: {} (size {})", info.map, info.map_size);

    socket.disconnect().await?;
    Ok(())
}
