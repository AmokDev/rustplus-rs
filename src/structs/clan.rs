use crate::proto::clan_info::{Invite, Member, Role};
use crate::proto::{AppClanChat, AppClanMessage, ClanInfo};

#[derive(Debug, Clone)]
pub struct RustClanInfo {
    pub clan_id: i64,
    pub name: String,
    pub created: i64,
    pub creator: u64,
    pub motd: String,
    pub motd_timestamp: i64,
    pub motd_author: u64,
    pub logo: Vec<u8>,
    pub color: i32,
    pub roles: Vec<RustRole>,
    pub members: Vec<RustMember>,
    pub invites: Vec<RustInvite>,
    pub max_member_count: i32,
}

#[derive(Debug, Clone)]
pub struct RustRole {
    pub role_id: i32,
    pub rank: i32,
    pub name: String,
    pub can_set_motd: bool,
    pub can_set_logo: bool,
    pub can_invite: bool,
    pub can_kick: bool,
    pub can_promote: bool,
    pub can_demote: bool,
    pub can_set_player_notes: bool,
    pub can_access_logs: bool,
}

#[derive(Debug, Clone)]
pub struct RustMember {
    pub steam_id: u64,
    pub role_id: i32,
    pub joined: i64,
    pub last_seen: i64,
    pub notes: String,
    pub online: bool,
}

#[derive(Debug, Clone)]
pub struct RustInvite {
    pub steam_id: u64,
    pub recruiter: u64,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct RustClanMessage {
    pub steam_id: u64,
    pub name: String,
    pub message: String,
    pub time: i64,
}

#[derive(Debug, Clone)]
pub struct RustClanChat {
    pub messages: Vec<RustClanMessage>,
}

impl From<ClanInfo> for RustClanInfo {
    fn from(clan_info: ClanInfo) -> Self {
        Self {
            clan_id: clan_info.clan_id,
            name: clan_info.name,
            created: clan_info.created,
            creator: clan_info.creator,
            motd: clan_info.motd,
            motd_timestamp: clan_info.motd_timestamp,
            motd_author: clan_info.motd_author,
            logo: clan_info.logo,
            color: clan_info.color,
            roles: clan_info
                .roles
                .into_iter()
                .map(RustRole::from)
                .collect(),
            members: clan_info
                .members
                .into_iter()
                .map(RustMember::from)
                .collect(),
            invites: clan_info
                .invites
                .into_iter()
                .map(RustInvite::from)
                .collect(),
            max_member_count: clan_info.max_member_count,
        }
    }
}

impl From<Role> for RustRole {
    fn from(role: Role) -> Self {
        Self {
            role_id: role.role_id,
            rank: role.rank,
            name: role.name,
            can_set_motd: role.can_set_motd,
            can_set_logo: role.can_set_logo,
            can_invite: role.can_invite,
            can_kick: role.can_kick,
            can_promote: role.can_promote,
            can_demote: role.can_demote,
            can_set_player_notes: role.can_set_player_notes,
            can_access_logs: role.can_access_logs,
        }
    }
}

impl From<Member> for RustMember {
    fn from(member: Member) -> Self {
        Self {
            steam_id: member.steam_id,
            role_id: member.role_id,
            joined: member.joined,
            last_seen: member.last_seen,
            notes: member.notes,
            online: member.online,
        }
    }
}

impl From<Invite> for RustInvite {
    fn from(invite: Invite) -> Self {
        Self {
            steam_id: invite.steam_id,
            recruiter: invite.recruiter,
            timestamp: invite.timestamp,
        }
    }
}

impl From<AppClanMessage> for RustClanMessage {
    fn from(clan_message: AppClanMessage) -> Self {
        Self {
            steam_id: clan_message.steam_id,
            name: clan_message.name,
            message: clan_message.message,
            time: clan_message.time,
        }
    }
}

impl From<AppClanChat> for RustClanChat {
    fn from(clan_chat: AppClanChat) -> Self {
        Self {
            messages: clan_chat
                .messages
                .into_iter()
                .map(RustClanMessage::from)
                .collect(),
        }
    }
}
