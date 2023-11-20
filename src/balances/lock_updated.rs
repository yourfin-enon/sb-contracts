service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "wallet-balance-lock-updated")]
pub struct BalanceLockUpdatedSbEvent {
    #[prost(string, tag = "1")]
    pub trader_id: String,
    #[prost(string, tag = "2")]
    pub wallet_id: String,
    #[prost(string, tag = "3")]
    pub balance_id: String,
    #[prost(bool, tag = "4")]
    pub is_locked: bool,
}