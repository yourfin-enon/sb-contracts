use std::collections::HashMap;
use my_service_bus_abstractions::{GetMySbModelTopicId, SubscriberError};
use my_service_bus_abstractions::publisher::MySbMessageSerializer;
use my_service_bus_abstractions::subscriber::MySbMessageDeserializer;
use crate::shared::{from_bytes, into_bytes};

pub const TOPIC_NAME: &str = "transaction-updated";

#[derive(Clone, PartialEq, ::prost::Message)]
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

impl GetMySbModelTopicId for TransactionUpdatedSbEvent {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for TransactionUpdatedSbEvent {
    fn serialize(
        &self,
        headers: Option<std::collections::HashMap<String, String>>,
    ) -> Result<(Vec<u8>, Option<std::collections::HashMap<String, String>>), String> {
        let content = into_bytes(self);

        match content {
            Ok(content) => Ok((content, headers)),
            Err(err) => Err(format!("{err}")),
        }
    }
}

impl MySbMessageDeserializer for TransactionUpdatedSbEvent {
    type Item = TransactionUpdatedSbEvent;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = from_bytes(src);

        match result {
            Ok(model) => Ok(model),
            Err(err) => Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}