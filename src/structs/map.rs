use crate::proto::app_map::Monument;
use crate::proto::app_marker::SellOrder;
use crate::proto::{AppMap, AppMapMarkers, AppMarker, Vector4};

#[derive(Debug, Clone)]
pub struct RustVector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone)]
pub struct RustMapMarkers {
    pub markers: Vec<RustMarker>,
}

#[derive(Debug, Clone)]
pub struct RustMarker {
    pub id: u32,
    pub r#type: i32,
    pub x: f32,
    pub y: f32,
    pub steam_id: u64,
    pub rotation: f32,
    pub radius: f32,
    pub color1: Option<RustVector4>,
    pub color2: Option<RustVector4>,
    pub alpha: f32,
    pub name: String,
    pub out_of_stock: bool,
    pub sell_orders: Vec<RustSellOrder>,
}

#[derive(Debug, Clone)]
pub struct RustSellOrder {
    pub item_id: i32,
    pub quantity: i32,
    pub currency_id: i32,
    pub cost_per_item: i32,
    pub amount_in_stock: i32,
    pub item_is_blueprint: bool,
    pub currency_is_blueprint: bool,
    pub item_condition: f32,
    pub item_condition_max: f32,
}

#[derive(Debug, Clone)]
pub struct RustMonument {
    pub token: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct RustMap {
    pub width: u32,
    pub height: u32,
    pub jpg_image: Vec<u8>,
    pub ocean_margin: i32,
    pub monuments: Vec<RustMonument>,
    pub background: String,
}

impl From<Vector4> for RustVector4 {
    fn from(vector4: Vector4) -> Self {
        Self {
            x: vector4.x,
            y: vector4.y,
            z: vector4.z,
            w: vector4.w,
        }
    }
}

impl From<AppMapMarkers> for RustMapMarkers {
    fn from(map_markers: AppMapMarkers) -> Self {
        Self {
            markers: map_markers
                .markers
                .into_iter()
                .map(RustMarker::from)
                .collect(),
        }
    }
}

impl From<AppMarker> for RustMarker {
    fn from(marker: AppMarker) -> Self {
        Self {
            id: marker.id,
            r#type: marker.r#type,
            x: marker.x,
            y: marker.y,
            steam_id: marker.steam_id,
            rotation: marker.rotation,
            radius: marker.radius,
            color1: marker.color1.map(RustVector4::from),
            color2: marker.color2.map(RustVector4::from),
            alpha: marker.alpha,
            name: marker.name,
            out_of_stock: marker.out_of_stock,
            sell_orders: marker
                .sell_orders
                .into_iter()
                .map(RustSellOrder::from)
                .collect(),
        }
    }
}

impl From<SellOrder> for RustSellOrder {
    fn from(sell_order: SellOrder) -> Self {
        Self {
            item_id: sell_order.item_id,
            quantity: sell_order.quantity,
            currency_id: sell_order.currency_id,
            cost_per_item: sell_order.cost_per_item,
            amount_in_stock: sell_order.amount_in_stock,
            item_is_blueprint: sell_order.item_is_blueprint,
            currency_is_blueprint: sell_order.currency_is_blueprint,
            item_condition: sell_order.item_condition,
            item_condition_max: sell_order.item_condition_max,
        }
    }
}

impl From<Monument> for RustMonument {
    fn from(m: Monument) -> Self {
        Self {
            token: m.token,
            x: m.x,
            y: m.y,
        }
    }
}

impl From<AppMap> for RustMap {
    fn from(map: AppMap) -> Self {
        Self {
            width: map.width,
            height: map.height,
            jpg_image: map.jpg_image,
            ocean_margin: map.ocean_margin,
            monuments: map.monuments.into_iter().map(RustMonument::from).collect(),
            background: map.background,
        }
    }
}
