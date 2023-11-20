use super::order::OrderSbModel;
service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "wallet-position-margin-call")]
pub struct PositionMarginCallSbEvent {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(message, optional, tag = "2")]
    pub order: Option<OrderSbModel>,
    #[prost(double, tag = "3")]
    pub pnl: f64,
    #[prost(double, tag = "4")]
    pub loss_percent: f64,
    #[prost(double, tag = "5")]
    pub total_invest_amount: f64,
}
