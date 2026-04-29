use crate::proto::app_entity_payload::Item;
use crate::proto::{AppEntityInfo, AppEntityPayload, AppEntityType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustEntityType {
    Unknown,
    Switch,
    Alarm,
    StorageMonitor,
}

#[derive(Debug, Clone)]
pub struct RustEntityItem {
    pub item_id: i32,
    pub quantity: i32,
    pub item_is_blueprint: bool,
}

#[derive(Debug, Clone)]
pub struct RustEntityPayload {
    pub value: bool,
    pub items: Vec<RustEntityItem>,
    pub capacity: i32,
    pub has_protection: bool,
    pub protection_expiry: u32,
}

#[derive(Debug, Clone)]
pub struct RustEntityInfo {
    pub entity_type: RustEntityType,
    pub payload: Option<RustEntityPayload>,
}

impl From<Item> for RustEntityItem {
    fn from(item: Item) -> Self {
        Self {
            item_id: item.item_id,
            quantity: item.quantity,
            item_is_blueprint: item.item_is_blueprint,
        }
    }
}

impl From<AppEntityType> for RustEntityType {
    fn from(t: AppEntityType) -> Self {
        match t {
            AppEntityType::UnknownType => RustEntityType::Unknown,
            AppEntityType::Switch => RustEntityType::Switch,
            AppEntityType::Alarm => RustEntityType::Alarm,
            AppEntityType::StorageMonitor => RustEntityType::StorageMonitor,
        }
    }
}

impl From<AppEntityPayload> for RustEntityPayload {
    fn from(payload: AppEntityPayload) -> Self {
        Self {
            value: payload.value,
            items: payload
                .items
                .into_iter()
                .map(RustEntityItem::from)
                .collect(),
            capacity: payload.capacity,
            has_protection: payload.has_protection,
            protection_expiry: payload.protection_expiry,
        }
    }
}

impl From<AppEntityInfo> for RustEntityInfo {
    fn from(info: AppEntityInfo) -> Self {
        let entity_type = AppEntityType::try_from(info.r#type)
            .unwrap_or(AppEntityType::UnknownType)
            .into();

        Self {
            entity_type,
            payload: info.payload.map(RustEntityPayload::from),
        }
    }
}
