use rustplus::RustSocket;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), rustplus::Error> {
    let mut socket = RustSocket::builder()
        .ip("YOUR_SERVER_IP")
        .port(28082)
        .player_id(76561199000000000)
        .player_token(123456)
        .build()?;

    socket.connect().await?;

    let start = Instant::now();
    println!("Sending 70 get_time requests...");

    for i in 1..=70 {
        let req_start = Instant::now();
        let _ = socket.get_time().await?;
        println!(
            "[{:3}] elapsed={:>6.0}ms total={:>6.2}s",
            i,
            req_start.elapsed().as_millis() as f64,
            start.elapsed().as_secs_f64()
        );
    }

    println!("\nTotal time: {:.2}s", start.elapsed().as_secs_f64());
    socket.disconnect().await?;
    Ok(())
}
