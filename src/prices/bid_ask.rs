use super::bid_ask_date_time::BidAskDateTime;
service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "bidask")]
pub struct BidAskSbModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, tag = "2")]
    pub date_time: ::core::option::Option<BidAskDateTime>,
    #[prost(double, tag = "3")]
    pub bid: f64,
    #[prost(double, tag = "4")]
    pub ask: f64,
    #[prost(double, tag = "5")]
    pub bid_volume: f64,
    #[prost(double, tag = "6")]
    pub ask_volume: f64,
}
