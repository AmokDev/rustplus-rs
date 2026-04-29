#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot connect to websocket")]
    WebsocketConnectionError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Protobuf decode error")]
    ProtobufDecodeError(#[from] prost::DecodeError),
    #[error("Protobuf encode error")]
    ProtobufEncodeError(#[from] prost::EncodeError),
    #[error("Websocket connection closed")]
    WebsocketConnectionClosed,
    #[error("Server error: {0}")]
    ServerError(String),
    #[error("Server response missing expected field: {0}")]
    MissingResponseField(&'static str),
    #[error("Socket is not connected. Call connect() first")]
    NotConnected,
    #[error("Builder missing required field: {0}")]
    BuilderMissingField(&'static str),
}
