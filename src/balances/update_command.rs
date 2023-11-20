service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "wallet-balances-update-commands")]
pub struct UpdateBalancesSbCommand {
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub date_micros: i64,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub updates: ::prost::alloc::vec::Vec<BalanceUpdateCommandSbModel>,
    #[prost(int32, tag = "5")]
    pub operation_type: i32,
    #[prost(string, tag = "6")]
    pub process_id: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdateCommandSbModel {
    #[prost(int32, tag = "1")]
    pub number: i32,
    #[prost(string, tag = "2")]
    pub asset_symbol: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub amount: f64,
    #[prost(string, tag = "4")]
    pub wallet_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub trader_id: ::prost::alloc::string::String,
    #[prost(double, tag = "6")]
    pub reserve_amount: f64,
    #[prost(int32, tag = "7")]
    pub update_reason: i32,
    #[prost(double, tag = "8")]
    pub bonus: f64,
    #[prost(double, tag = "9")]
    pub reserve_bonus: f64,
}
