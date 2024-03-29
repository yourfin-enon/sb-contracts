service_sdk::macros::use_my_sb_entity_protobuf_model!();
use crate::balances::shared::{BalanceSbModel, BalanceUpdateInfoSbModel, BalanceUpdateSbModel};

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "wallet-balances-updated")]
pub struct BalancesUpdatedSbEvent {
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
    pub update_infos: ::prost::alloc::vec::Vec<BalanceUpdateInfoSbModel>,
}

impl BalancesUpdatedSbEvent {
    pub fn as_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let version: u8 = 0;
        let mut result = vec![version];
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(&bytes[1..])
    }
}
