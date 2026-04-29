use crate::proto::app_team_info::{Member, Note};
use crate::proto::{AppTeamChat, AppTeamInfo, AppTeamMessage};

#[derive(Debug, Clone)]
pub struct RustTeamMember {
    pub steam_id: u64,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub is_online: bool,
    pub spawn_time: u32,
    pub is_alive: bool,
    pub death_time: u32,
}

#[derive(Debug, Clone)]
pub struct RustTeamNote {
    pub r#type: i32,
    pub x: f32,
    pub y: f32,
    pub icon: i32,
    pub colour_index: i32,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct RustTeamInfo {
    pub leader_steam_id: u64,
    pub members: Vec<RustTeamMember>,
    pub map_notes: Vec<RustTeamNote>,
    pub leader_map_notes: Vec<RustTeamNote>,
}

#[derive(Debug, Clone)]
pub struct RustTeamMessage {
    pub steam_id: u64,
    pub name: String,
    pub message: String,
    pub color: String,
    pub time: u32,
}

#[derive(Debug, Clone)]
pub struct RustTeamChat {
    pub messages: Vec<RustTeamMessage>,
}

impl From<Member> for RustTeamMember {
    fn from(team_member: Member) -> Self {
        Self {
            steam_id: team_member.steam_id,
            name: team_member.name,
            x: team_member.x,
            y: team_member.y,
            is_online: team_member.is_online,
            spawn_time: team_member.spawn_time,
            is_alive: team_member.is_alive,
            death_time: team_member.death_time,
        }
    }
}

impl From<Note> for RustTeamNote {
    fn from(team_note: Note) -> Self {
        Self {
            r#type: team_note.r#type,
            x: team_note.x,
            y: team_note.y,
            icon: team_note.icon,
            colour_index: team_note.colour_index,
            label: team_note.label,
        }
    }
}

impl From<AppTeamInfo> for RustTeamInfo {
    fn from(team_info: AppTeamInfo) -> Self {
        Self {
            leader_steam_id: team_info.leader_steam_id,
            members: team_info
                .members
                .into_iter()
                .map(RustTeamMember::from)
                .collect(),
            map_notes: team_info
                .map_notes
                .into_iter()
                .map(RustTeamNote::from)
                .collect(),
            leader_map_notes: team_info
                .leader_map_notes
                .into_iter()
                .map(RustTeamNote::from)
                .collect(),
        }
    }
}

impl From<AppTeamMessage> for RustTeamMessage {
    fn from(team_message: AppTeamMessage) -> Self {
        Self {
            steam_id: team_message.steam_id,
            name: team_message.name,
            message: team_message.message,
            color: team_message.color,
            time: team_message.time,
        }
    }
}

impl From<AppTeamChat> for RustTeamChat {
    fn from(team_chat: AppTeamChat) -> Self {
        Self {
            messages: team_chat
                .messages
                .into_iter()
                .map(RustTeamMessage::from)
                .collect(),
        }
    }
}
