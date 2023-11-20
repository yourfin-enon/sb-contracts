service_sdk::macros::use_my_sb_entity_protobuf_model!();
use super::order::OrderSbModel;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "wallet-order-updated")]
pub struct OrderUpdatedSbEvent {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<OrderSbModel>,
    #[prost(string, tag = "2")]
    pub position_id: String,
}
