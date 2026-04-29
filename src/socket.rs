use futures_util::SinkExt;
use futures_util::StreamExt;
use futures_util::stream::{SplitSink, SplitStream};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use prost::Message;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, oneshot};
use tokio_tungstenite::tungstenite::Message::Binary;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::error::Error;
use crate::proto::{
    AppBroadcast, AppEmpty, AppFlag, AppMessage, AppPromoteToLeader, AppRequest, AppResponse,
    AppSendMessage, AppSetEntityValue,
};
use crate::rate_limiter::RateLimiter;
use crate::structs::clan::{RustClanChat, RustClanInfo};
use crate::structs::entity::RustEntityInfo;
use crate::structs::map::{RustMap, RustMapMarkers};
use crate::structs::server_info::RustInfo;
use crate::structs::team::{RustTeamChat, RustTeamInfo};
use crate::structs::time::RustTime;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WriteHalf = SplitSink<WsStream, tokio_tungstenite::tungstenite::Message>;
type ReadHalf = SplitStream<WsStream>;
type PendingMap = Arc<Mutex<HashMap<u32, oneshot::Sender<AppResponse>>>>;

async fn reader_loop(
    mut read_half: ReadHalf,
    pending: PendingMap,
    broadcast_tx: broadcast::Sender<AppBroadcast>,
) {
    loop {
        let msg_result = read_half.next().await;

        let msg = match msg_result {
            Some(Ok(msg)) => msg,
            Some(Err(_)) => break,
            None => break,
        };

        let buf = msg.into_data();

        let app_message = AppMessage::decode(buf.as_ref());
        let app_message = match app_message {
            Ok(m) => m,
            Err(_) => continue,
        };

        if let Some(response) = app_message.response
            && let Some(tx) = pending
                .lock()
                .expect("pending mutex poisoned")
                .remove(&response.seq)
        {
            let _ = tx.send(response);
        }

        if let Some(broadcast) = app_message.broadcast {
            let _ = broadcast_tx.send(broadcast);
        }
    }
}

#[derive(Debug)]
pub struct RustSocket {
    seq: u32,
    ip: String,
    port: u16,
    player_id: u64,
    player_token: i32,
    write_half: Option<WriteHalf>,
    pending: PendingMap,
    broadcast_tx: broadcast::Sender<AppBroadcast>,
    server_rate_limiter: Arc<Mutex<RateLimiter>>,
    socket_rate_limiter: Arc<Mutex<RateLimiter>>,
}

pub struct RustSocketBuilder {
    ip: Option<String>,
    port: Option<u16>,
    player_id: Option<u64>,
    player_token: Option<i32>,
}

impl RustSocketBuilder {
    fn new() -> Self {
        Self {
            ip: None,
            port: None,
            player_id: None,
            player_token: None,
        }
    }

    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn player_id(mut self, id: u64) -> Self {
        self.player_id = Some(id);
        self
    }

    pub fn player_token(mut self, token: i32) -> Self {
        self.player_token = Some(token);
        self
    }

    pub fn build(self) -> Result<RustSocket, Error> {
        let ip = self.ip.ok_or(Error::BuilderMissingField("ip"))?;
        let port = self.port.ok_or(Error::BuilderMissingField("port"))?;
        let player_id = self
            .player_id
            .ok_or(Error::BuilderMissingField("player_id"))?;
        let player_token = self
            .player_token
            .ok_or(Error::BuilderMissingField("player_token"))?;

        let (broadcast_tx, _) = broadcast::channel(64);

        Ok(RustSocket {
            seq: 1,
            ip,
            port,
            player_id,
            player_token,
            write_half: None,
            pending: Arc::new(Mutex::new(HashMap::new())),
            broadcast_tx,
            server_rate_limiter: Arc::new(Mutex::new(RateLimiter::new(50.0, 15.0))),
            socket_rate_limiter: Arc::new(Mutex::new(RateLimiter::new(15.0, 2.0))),
        })
    }
}

impl RustSocket {
    pub fn builder() -> RustSocketBuilder {
        RustSocketBuilder::new()
    }

    pub fn new(ip: String, port: u16, player_id: u64, player_token: i32) -> Self {
        Self::builder()
            .ip(ip)
            .port(port)
            .player_id(player_id)
            .player_token(player_token)
            .build()
            .expect("builder has all required fields")
    }

    pub async fn connect(&mut self) -> Result<(), crate::error::Error> {
        let url = format!("ws://{}:{}", self.ip, self.port);
        let (stream, _) = tokio_tungstenite::connect_async(&url).await?;
        let (write, read) = stream.split();
        self.write_half = Some(write);
        tokio::spawn(reader_loop(
            read,
            self.pending.clone(),
            self.broadcast_tx.clone(),
        ));

        Ok(())
    }

    async fn wait_for_tokens(&self, cost: f64) {
        loop {
            let wait = {
                let mut server = self
                    .server_rate_limiter
                    .lock()
                    .expect("server rate_limiter mutex poisoned");
                let mut socket = self
                    .socket_rate_limiter
                    .lock()
                    .expect("socket rate_limiter mutex poisoned");

                let server_wait = server.check_wait(cost);
                let socket_wait = socket.check_wait(cost);
                let wait = server_wait.max(socket_wait);

                if wait.is_zero() {
                    server.consume(cost);
                    socket.consume(cost);
                    return;
                }

                wait
            };

            tokio::time::sleep(wait).await;
        }
    }

    pub async fn send_request(
        &mut self,
        request: AppRequest,
        cost: f64,
    ) -> Result<(oneshot::Receiver<AppResponse>, u32), Error> {
        self.wait_for_tokens(cost).await;

        let seq = self.seq;

        let mut buf: Vec<u8> = Vec::new();
        request.encode(&mut buf)?;
        let stream = self.write_half.as_mut().ok_or(Error::NotConnected)?;
        stream.send(Binary(buf.into())).await?;

        let (tx, rx) = oneshot::channel();
        self.pending
            .lock()
            .expect("pending mutex poisoned")
            .insert(seq, tx);

        self.seq += 1;
        Ok((rx, seq))
    }

    async fn send_and_wait(
        &mut self,
        request: AppRequest,
        cost: f64,
    ) -> Result<AppResponse, Error> {
        loop {
            let (rx, seq) = self.send_request(request.clone(), cost).await?;

            let response = match rx.await {
                Ok(r) => r,
                Err(_) => {
                    self.pending
                        .lock()
                        .expect("pending mutex poisoned")
                        .remove(&seq);
                    return Err(Error::WebsocketConnectionClosed);
                }
            };

            if let Some(err) = &response.error {
                if err.error == "rate_limit" {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    continue;
                }
                return Err(Error::ServerError(err.error.clone()));
            }

            return Ok(response);
        }
    }

    #[deprecated(
        note = "Use events() instead for typed RustEvent. Use this only if you need raw protobuf AppBroadcast"
    )]
    pub fn subscribe_raw(&self) -> broadcast::Receiver<AppBroadcast> {
        self.broadcast_tx.subscribe()
    }

    pub fn events(&self) -> crate::events::EventListener {
        crate::events::EventListener::new(self.broadcast_tx.subscribe())
    }

    pub async fn disconnect(&mut self) -> Result<(), Error> {
        if let Some(mut write) = self.write_half.take() {
            write.close().await?;
        }

        Ok(())
    }

    pub async fn get_info(&mut self) -> Result<RustInfo, Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            get_info: Some(AppEmpty {}),
            ..Default::default()
        };

        let response = self.send_and_wait(request, 1.0).await?;
        let app_info = response.info.ok_or(Error::MissingResponseField("info"))?;
        let rust_info = RustInfo::from(app_info);
        Ok(rust_info)
    }

    pub async fn get_time(&mut self) -> Result<RustTime, Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            get_time: Some(AppEmpty {}),
            ..Default::default()
        };

        let response = self.send_and_wait(request, 1.0).await?;
        let app_info = response.time.ok_or(Error::MissingResponseField("time"))?;
        let rust_time = RustTime::from(app_info);
        Ok(rust_time)
    }

    pub async fn get_team_info(&mut self) -> Result<RustTeamInfo, Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            get_team_info: Some(AppEmpty {}),
            ..Default::default()
        };

        let response = self.send_and_wait(request, 1.0).await?;
        let app_info = response
            .team_info
            .ok_or(Error::MissingResponseField("team_info"))?;
        let rust_team_info = RustTeamInfo::from(app_info);
        Ok(rust_team_info)
    }

    pub async fn get_team_chat(&mut self) -> Result<RustTeamChat, Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            get_team_chat: Some(AppEmpty {}),
            ..Default::default()
        };

        let response = self.send_and_wait(request, 1.0).await?;
        let app_info = response
            .team_chat
            .ok_or(Error::MissingResponseField("team_chat"))?;
        let team_chat = RustTeamChat::from(app_info);
        Ok(team_chat)
    }

    pub async fn get_map_markers(&mut self) -> Result<RustMapMarkers, Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            get_map_markers: Some(AppEmpty {}),
            ..Default::default()
        };

        let response = self.send_and_wait(request, 1.0).await?;
        let app_info = response
            .map_markers
            .ok_or(Error::MissingResponseField("map_markers"))?;
        let map_markers = RustMapMarkers::from(app_info);
        Ok(map_markers)
    }

    pub async fn get_clan_info(&mut self) -> Result<RustClanInfo, Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            get_clan_info: Some(AppEmpty {}),
            ..Default::default()
        };

        let response = self.send_and_wait(request, 1.0).await?;
        let app_info = response
            .clan_info
            .ok_or(Error::MissingResponseField("clan_info"))?;
        let clan_info = RustClanInfo::from(
            app_info
                .clan_info
                .ok_or(Error::MissingResponseField("clan_info.clan_info"))?,
        );
        Ok(clan_info)
    }

    pub async fn get_clan_chat(&mut self) -> Result<RustClanChat, Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            get_clan_chat: Some(AppEmpty {}),
            ..Default::default()
        };

        let response = self.send_and_wait(request, 1.0).await?;
        let app_info = response
            .clan_chat
            .ok_or(Error::MissingResponseField("clan_chat"))?;
        let clan_chat = RustClanChat::from(app_info);
        Ok(clan_chat)
    }

    pub async fn get_entity_info(&mut self, entity_id: u32) -> Result<RustEntityInfo, Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            entity_id,
            get_entity_info: Some(AppEmpty {}),
            ..Default::default()
        };

        let response = self.send_and_wait(request, 1.0).await?;
        let app_info = response
            .entity_info
            .ok_or(Error::MissingResponseField("entity_info"))?;
        let entity_info = RustEntityInfo::from(app_info);
        Ok(entity_info)
    }

    pub async fn set_entity_value(&mut self, entity_id: u32, value: bool) -> Result<(), Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            entity_id,
            set_entity_value: Some(AppSetEntityValue { value }),
            ..Default::default()
        };

        self.send_and_wait(request, 1.0).await?;
        Ok(())
    }

    pub async fn check_subscription(&mut self, entity_id: u32) -> Result<bool, Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            entity_id,
            check_subscription: Some(AppEmpty {}),
            ..Default::default()
        };

        let response = self.send_and_wait(request, 1.0).await?;
        let flag = response.flag.ok_or(Error::MissingResponseField("flag"))?;
        Ok(flag.value)
    }

    pub async fn set_subscription(&mut self, entity_id: u32, value: bool) -> Result<(), Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            entity_id,
            set_subscription: Some(AppFlag { value }),
            ..Default::default()
        };

        self.send_and_wait(request, 1.0).await?;
        Ok(())
    }

    pub async fn promote_to_leader(&mut self, steam_id: u64) -> Result<(), Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            promote_to_leader: Some(AppPromoteToLeader { steam_id }),
            ..Default::default()
        };

        self.send_and_wait(request, 1.0).await?;
        Ok(())
    }

    pub async fn send_team_message(&mut self, message: String) -> Result<(), Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            send_team_message: Some(AppSendMessage { message }),
            ..Default::default()
        };

        self.send_and_wait(request, 2.0).await?;
        Ok(())
    }

    pub async fn send_clan_message(&mut self, message: String) -> Result<(), Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            send_clan_message: Some(AppSendMessage { message }),
            ..Default::default()
        };

        self.send_and_wait(request, 2.0).await?;
        Ok(())
    }

    pub async fn set_clan_motd(&mut self, message: String) -> Result<(), Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            set_clan_motd: Some(AppSendMessage { message }),
            ..Default::default()
        };

        self.send_and_wait(request, 2.0).await?;
        Ok(())
    }

    pub async fn get_map(&mut self) -> Result<RustMap, Error> {
        let request = AppRequest {
            seq: self.seq,
            player_id: self.player_id,
            player_token: self.player_token,
            get_map: Some(AppEmpty {}),
            ..Default::default()
        };

        let response = self.send_and_wait(request, 5.0).await?;
        let map = response.map.ok_or(Error::MissingResponseField("map"))?;
        Ok(RustMap::from(map))
    }
}
