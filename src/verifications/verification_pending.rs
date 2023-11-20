use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "verification-pending")]
pub struct VerificationPendingSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    #[prost(string, tag = "3")]
    pub code: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub reason: i32,
    #[prost(optional, string, tag = "5")]
    pub additional_data_json: Option<String>,
    #[prost(int32, tag = "6")]
    pub lifetime_sec: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalVerificationAdditionalDataSbModel {
    pub address: String,
    pub asset_amount: f64,
    pub asset_symbol: String,
    pub blockchain_symbol: String,
}
