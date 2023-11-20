use super::order::OrderSbModel;
service_sdk::macros::use_my_sb_entity_protobuf_model!();
use std::collections::HashMap;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "wallet-position-pending")]
pub struct PositionPendingSbEvent {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(message, optional, tag = "2")]
    pub order: Option<OrderSbModel>,
    #[prost(int64, tag = "3")]
    pub open_date_micros: i64,
    #[prost(map = "string, double", tag = "4")]
    pub open_asset_prices: HashMap<String, f64>,
    #[prost(double, tag = "5")]
    pub open_price: f64,
}
