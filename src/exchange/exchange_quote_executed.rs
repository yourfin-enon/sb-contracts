service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "exchange-quote-executed")]
pub struct ExchangeQuoteExecutedSbEvent {
    #[prost(string, tag = "1")]
    pub trader_id: String,
    #[prost(string, tag = "2")]
    pub wallet_id: String,
    #[prost(string, tag = "3")]
    pub balance_id: String,
    #[prost(string, tag = "4")]
    pub id: String,
    #[prost(int64, tag = "5")]
    pub created_date_micros: i64,
    #[prost(int64, tag = "6")]
    pub executed_date_micros: i64,
    #[prost(double, tag = "7")]
    pub from_asset_amount: f64,
    #[prost(string, tag = "8")]
    pub from_asset_symbol: String,
    #[prost(string, tag = "9")]
    pub to_asset_symbol: String,
    #[prost(double, tag = "10")]
    pub to_asset_amount: f64,
    #[prost(double, tag = "11")]
    pub req_asset_amount: f64,
    #[prost(string, tag = "12")]
    pub req_asset_symbol: String,
    #[prost(double, tag = "13")]
    pub price: f64,
    #[prost(string, tag = "14")]
    pub fee_asset_symbol: String,
    #[prost(double, tag = "15")]
    pub fee_asset_amount: f64,
    #[prost(int32, tag = "16")]
    pub quote_type: i32,
    #[prost(int32, tag = "17")]
    pub req_scene: i32,
    #[prost(string, tag = "18")]
    pub operation_id: String,
}