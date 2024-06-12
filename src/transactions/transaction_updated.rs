use crate::transactions::shared::transaction_sb_event;
service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "transaction-updated")]
pub struct TransactionUpdatedSbEvent {
    #[prost(int64, tag = "1")]
    pub date_micros: i64,
    #[prost(string, tag = "2")]
    pub asset_symbol: String,
    #[prost(string, tag = "3")]
    pub tx_id: String,
    #[prost(string, tag = "4")]
    pub destination_address: String,
    #[prost(string, optional, tag = "5")]
    pub source_address: Option<String>,
    #[prost(string, tag = "6")]
    pub wallet_id: String,
    #[prost(int32, tag = "7")]
    pub transaction_type: i32,
    #[prost(double, tag = "8")]
    pub asset_amount: f64,
    #[prost(string, tag = "9")]
    pub trader_id: String,
    #[prost(string, tag = "10")]
    pub blockchain_symbol: String,
    #[prost(double, tag = "11")]
    pub fee_amount: f64,
    #[prost(int32, tag = "12")]
    pub status: i32,
    #[prost(string, tag = "13")]
    pub external_id: String,
    #[prost(int32, tag = "14")]
    pub payment_provider: i32,
    #[prost(string, tag = "15")]
    pub id: String,
    #[prost(message, optional, tag = "16")]
    pub prev_state: Option<TransactionPrevStateSbModel>,
    #[prost(message, optional, tag = "17")]
    pub convert: Option<TransactionConvertSbModel>
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionPrevStateSbModel {
    #[prost(int64, tag = "1")]
    pub last_update_ts_micros: i64,
    #[prost(string, tag = "2")]
    pub tx_id: String,
    #[prost(int32, tag = "3")]
    pub status: i32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionConvertSbModel {
    #[prost(string, tag = "1")]
    pub fiat_asset_symbol: String,
    #[prost(string, tag = "2")]
    pub crypto_asset_symbol: String,
    #[prost(double, tag = "3")]
    pub fiat_asset_amount: f64,
    #[prost(double, tag = "4")]
    pub crypto_asset_amount: f64,
}
