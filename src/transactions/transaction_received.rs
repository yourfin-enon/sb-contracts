use std::collections::HashMap;
use my_service_bus_abstractions::{GetMySbModelTopicId, SubscriberError};
use my_service_bus_abstractions::publisher::MySbMessageSerializer;
use my_service_bus_abstractions::subscriber::MySbMessageDeserializer;

pub const TOPIC_NAME: &str = "transaction-received";

#[derive(Clone, PartialEq, ::prost::Message)]
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
}

impl TransactionReceivedSbEvent {
    fn as_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}

impl GetMySbModelTopicId for TransactionReceivedSbEvent {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for TransactionReceivedSbEvent {
    fn serialize(
        &self,
        headers: Option<std::collections::HashMap<String, String>>,
    ) -> Result<(Vec<u8>, Option<std::collections::HashMap<String, String>>), String> {
        let content = self.as_bytes();

        return match content {
            Ok(content) => Ok((content, headers)),
            Err(err) => Err(format!("{err}")),
        }
    }
}

impl MySbMessageDeserializer for TransactionReceivedSbEvent {
    type Item = TransactionReceivedSbEvent;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = TransactionReceivedSbEvent::from_bytes(src);

        return match result {
            Ok(model) => Ok(model),
            Err(err) => Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}