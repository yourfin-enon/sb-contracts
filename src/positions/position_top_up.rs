use super::order::OrderSbModel;
service_sdk::macros::use_my_sb_entity_protobuf_model!();
use std::collections::HashMap;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "wallet-position-top-up")]
pub struct PositionTopUpSbEvent {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(message, optional, tag = "2")]
    pub order: Option<OrderSbModel>,
    #[prost(message, repeated, tag = "3")]
    pub active_top_ups: Vec<ActiveTopUpSbModel>,
    #[prost(message, repeated, tag = "4")]
    pub canceled_top_ups: Vec<CanceledTopUpSbModel>,
    #[prost(map = "string, double", tag = "5")]
    pub total_invest_assets: std::collections::HashMap<String, f64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveTopUpSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(int64, tag = "2")]
    pub date_micros: i64,
    #[prost(map = "string, double", tag = "3")]
    pub assets: HashMap<String, f64>,
    #[prost(double, tag = "4")]
    pub instrument_price: f64,
    #[prost(map = "string, double", tag = "5")]
    pub asset_prices: HashMap<String, f64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanceledTopUpSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(int64, tag = "2")]
    pub date_micros: i64,
    #[prost(map = "string, double", tag = "3")]
    pub assets: HashMap<String, f64>,
    #[prost(double, tag = "4")]
    pub instrument_price: f64,
    #[prost(map = "string, double", tag = "5")]
    pub asset_prices: HashMap<String, f64>,
    #[prost(double, tag = "6")]
    pub cancel_instrument_price: f64,
    #[prost(int64, tag = "7")]
    pub cancel_date_micros: i64,
}