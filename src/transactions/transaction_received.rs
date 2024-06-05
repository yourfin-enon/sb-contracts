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
    #[prost(string, optional, tag = "21")]
    pub payment_method: Option<String>,
}

#[cfg(test)]
mod tests {
    use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;
    use crate::transactions::shared::transaction_sb_event::{AdditionalInfo, TransactionCryptoBuyInfoSb};
    use crate::transactions::transaction_received::TransactionReceivedSbEvent;

    #[test]
    fn ser_der() {
        let model = TransactionReceivedSbEvent {
            date_micros: DateTimeAsMicroseconds::now().unix_microseconds,
            asset_symbol: "test_asset_symbol".to_string(),
            tx_id: "test_tx".to_string(),
            destination_address: "destination_address".to_string(),
            source_address: None,
            wallet_id: "wallet_id".to_string(),
            transaction_type: 0,
            asset_amount: 0.0,
            trader_id: "trader_id".to_string(),
            blockchain_symbol: "blockchain_symbol".to_string(),
            payment_provider_fee: 0.0,
            status: 0,
            external_id: "external_id".to_string(),
            payment_provider: 0,
            memo: None,
            internal_fee: None,
            order_id: None,
            ref_tx_ids: vec![],
            provider_fee_details_json: None,
            additional_info: Some(AdditionalInfo::CryptBuyInfo(TransactionCryptoBuyInfoSb {
                payment_method: "payment_method".to_string(),
                fail_reason: "payment_method".to_string(),
                crypto_asset_amount: 1.0,
                crypto_asset_symbol: "crypto_asset_symbol".to_string(),
                fiat_asset_amount: 1.0,
                fiat_asset_symbol: "fiat_asset_symbol".to_string(),
                tap_fee_asset_amount: 0.1,
                tap_fee_asset_symbol: "tap_fee_asset_symbol".to_string(),
            })),
        };
        let bytes = model.as_protobuf_bytes().unwrap();
        assert!(!bytes.is_empty());
        let parsed = TransactionReceivedSbEvent::from_protobuf_bytes(&bytes).unwrap();
        assert_eq!(parsed.date_micros, model.date_micros);
    }
}