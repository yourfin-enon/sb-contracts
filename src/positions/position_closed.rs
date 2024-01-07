service_sdk::macros::use_my_sb_entity_protobuf_model!();
use std::collections::HashMap;
use crate::positions::order::OrderSbModel;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "wallet-position-closed")]
pub struct PositionClosedSbEvent {
    #[prost(string, tag = "1")]
    pub id: String,

    #[prost(message, optional, tag = "2")]
    pub order: Option<OrderSbModel>,

    #[prost(double, optional, tag = "3")]
    pub pnl: Option<f64>,
    
    #[prost(map = "string, double", tag = "4")]
    pub asset_pnls: HashMap<String, f64>,

    #[prost(int64, tag = "5")]
    pub close_date_micros: i64,

    #[prost(double, tag = "6")]
    pub close_price: f64,

    #[prost(map = "string, double", tag = "7")]
    pub close_asset_prices: HashMap<String, f64>,

    #[prost(enumeration = "PositionCloseReasonSb", tag = "8")]
    pub reason: i32,

    #[prost(int32, tag = "9")]
    pub status: i32,

    #[prost(double, optional, tag = "10")]
    pub activate_price: Option<f64>,

    #[prost(double, tag = "11")]
    pub total_invest_amount: f64,

    #[prost(map = "string, double", tag = "12")]
    pub bonus_invest_assets: HashMap<String, f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
#[repr(i32)]
pub enum PositionCloseReasonSb {
    ClientCommand = 0,
    StopOut = 1,
    TakeProfit = 2,
    StopLoss = 3,
    AdminCommand = 4,
}
