use crate::proto::{AppBroadcast, AppCameraRays};

use crate::structs::clan::{RustClanInfo, RustClanMessage};
use crate::structs::entity::RustEntityPayload;
use crate::structs::team::{RustTeamInfo, RustTeamMessage};

#[derive(Debug, Clone)]
pub enum RustEvent {
    TeamChanged {
        player_id: u64,
        team_info: RustTeamInfo,
    },
    TeamMessage(RustTeamMessage),
    EntityChanged {
        entity_id: u32,
        payload: RustEntityPayload,
    },
    ClanChanged(RustClanInfo),
    ClanMessage {
        clan_id: i64,
        message: RustClanMessage,
    },
    CameraRays(AppCameraRays),
}

pub(crate) fn into_event(broadcast: AppBroadcast) -> Option<RustEvent> {
    if let Some(team_changed) = broadcast.team_changed {
        return Some(RustEvent::TeamChanged {
            player_id: team_changed.player_id,
            team_info: team_changed.team_info?.into(),
        });
    }

    if let Some(team_message) = broadcast.team_message {
        return Some(RustEvent::TeamMessage(team_message.message?.into()));
    }

    if let Some(entity_changed) = broadcast.entity_changed {
        return Some(RustEvent::EntityChanged {
            entity_id: entity_changed.entity_id,
            payload: entity_changed.payload?.into(),
        });
    }

    if let Some(clan_changed) = broadcast.clan_changed {
        return Some(RustEvent::ClanChanged(clan_changed.clan_info?.into()));
    }

    if let Some(clan_message) = broadcast.clan_message {
        return Some(RustEvent::ClanMessage {
            clan_id: clan_message.clan_id,
            message: clan_message.message?.into(),
        });
    }

    if let Some(camera_rays) = broadcast.camera_rays {
        return Some(RustEvent::CameraRays(camera_rays));
    }

    None
}

pub struct EventListener {
    rx: tokio::sync::broadcast::Receiver<AppBroadcast>,
}

impl EventListener {
    pub(crate) fn new(rx: tokio::sync::broadcast::Receiver<AppBroadcast>) -> Self {
        Self { rx }
    }

    pub async fn next(&mut self) -> Option<RustEvent> {
        loop {
            match self.rx.recv().await {
                Ok(broadcast) => {
                    if let Some(event) = into_event(broadcast) {
                        return Some(event);
                    }
                }
                Err(_) => return None,
            }
        }
    }
}
