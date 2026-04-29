use rustplus::socket::RustSocket;

#[tokio::main]
async fn main() {
    let mut socket = RustSocket::builder()
        .ip("185.218.137.36")
        .port(28082)
        .player_id(76561199482541396)
        .player_token(-1685981582)
        .build()
        .unwrap();

    socket.connect().await.unwrap();
    // let mut events = socket.events();

    let info = socket.get_info().await.unwrap();

    println!("{:?}", info);

    // tokio::signal::ctrl_c().await.unwrap();
}
