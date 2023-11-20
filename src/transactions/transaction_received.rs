use crate::transactions::shared::transaction_sb_event;
service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "transaction-received")]
pub struct TransactionReceivedSbEvent {
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
    pub payment_provider_fee: f64,
    #[prost(int32, tag = "12")]
    pub status: i32,
    #[prost(string, tag = "13")]
    pub external_id: String,
    #[prost(int32, tag = "14")]
    pub payment_provider: i32,
    #[prost(string, optional, tag = "15")]
    pub memo: Option<String>,
    #[prost(double, optional, tag = "16")]
    pub internal_fee: Option<f64>,
    #[prost(string, optional, tag = "17")]
    pub order_id: Option<String>,
    #[prost(string, repeated, tag = "18")]
    pub ref_tx_ids: Vec<String>,
    #[prost(string, optional, tag = "19")]
    pub provider_fee_details_json: Option<String>,
    #[prost(oneof = "transaction_sb_event::AdditionalInfo", tags = "20")]
    pub additional_info: ::core::option::Option<transaction_sb_event::AdditionalInfo>,
}
