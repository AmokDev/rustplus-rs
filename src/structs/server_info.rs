use crate::proto::AppInfo;

#[derive(Debug, Clone)]
pub struct RustInfo {
    pub name: String,
    pub header_image: String,
    pub url: String,
    pub map: String,
    pub map_size: u32,
    pub wipe_time: u32,
    pub players: u32,
    pub max_players: u32,
    pub queued_players: u32,
    pub seed: u32,
    pub logo_image: String,
}

impl From<AppInfo> for RustInfo {
    fn from(info: AppInfo) -> Self {
        Self {
            name: info.name,
            header_image: info.header_image,
            url: info.url,
            map: info.map,
            map_size: info.map_size,
            wipe_time: info.wipe_time,
            players: info.players,
            max_players: info.max_players,
            queued_players: info.queued_players,
            seed: info.seed,
            logo_image: info.logo_image,
        }
    }
}
