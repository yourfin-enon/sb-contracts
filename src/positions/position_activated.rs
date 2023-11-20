use std::collections::HashMap;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model_with_version;
use super::order::OrderSbModel;
service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "wallet-position-activated")]
pub struct PositionActivedSbEvent {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<OrderSbModel>,
    #[prost(double, tag = "3")]
    pub activate_price: f64,
    #[prost(int64, tag = "4")]
    pub activate_date_micros: i64,
    #[prost(map = "string, double", tag = "5")]
    pub activate_asset_prices: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    #[prost(int64, tag = "6")]
    pub open_date_micros: i64,
    #[prost(map = "string, double", tag = "7")]
    pub open_asset_prices: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    #[prost(double, tag = "8")]
    pub open_price: f64,
    #[prost(map = "string, double", tag = "9")]
    pub total_invest_assets: HashMap<String, f64>,
    #[prost(map = "string, double", tag = "10")]
    pub invest_bonus_assets: HashMap<String, f64>,
}
