service_sdk::macros::use_my_sb_entity_protobuf_model!();
use crate::balances::shared::{BalanceSbModel, BalanceUpdateInfoSbModel, BalanceUpdateSbModel};

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "wallet-balances-update-failed")]
pub struct BalancesUpdateFailedSbEvent {
    #[prost(string, tag = "1")]
    pub operation_id: String,
    #[prost(int64, tag = "2")]
    pub date_micros: i64,
    #[prost(string, tag = "3")]
    pub description: String,
    #[prost(message, repeated, tag = "4")]
    pub balances: ::prost::alloc::vec::Vec<BalanceSbModel>,
    #[prost(int32, tag = "5")]
    pub operation_type: i32,
    #[prost(message, repeated, tag = "6")]
    pub updates: ::prost::alloc::vec::Vec<BalanceUpdateSbModel>,
    #[prost(message, repeated, tag = "7")]
    pub update_infos: Vec<BalanceUpdateInfoSbModel>,
}
