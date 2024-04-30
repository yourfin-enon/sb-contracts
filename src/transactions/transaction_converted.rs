service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "transaction-converted")]
pub struct TransactionConvertedSbEvent {
    #[prost(string, tag = "1")]
    pub convert_id: String,
    #[prost(string, tag = "2")]
    pub trader_id: String,
    #[prost(int64, tag = "3")]
    pub create_date_micros: i64,
    #[prost(string, tag = "4")]
    pub fiat_asset: String,
    #[prost(string, tag = "5")]
    pub crypto_asset: String,
    #[prost(double, tag = "6")]
    pub fiat_amount: f64,
    #[prost(double, tag = "7")]
    pub crypto_amount: f64,
    #[prost(string, tag = "8")]
    pub transaction_id: String,
}


