use std::collections::HashMap;
use my_service_bus_abstractions::{GetMySbModelTopicId, SubscriberError};
use my_service_bus_abstractions::publisher::MySbMessageSerializer;
use my_service_bus_abstractions::subscriber::MySbMessageDeserializer;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const TOPIC_NAME: &str = "transaction-received";

#[derive(Debug, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum TransactionTypeSb {
    Deposit = 0,
    Withdrawal = 1,
}

#[derive(Debug, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum TransactionStatusSb {
    Pending = 0,
    Approved = 1,
    Failed = 2,
}

#[derive(Debug, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum PaymentProvider {
    CoinsPaid = 0,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(int64, tag = "2")]
    pub date_micros: i64,
    #[prost(string, tag = "3")]
    pub asset_symbol: String,
    #[prost(string, tag = "4")]
    pub tx_id: String,
    #[prost(string, tag = "5")]
    pub destination_address: String,
    #[prost(string, optional, tag = "6")]
    pub source_address: Option<String>,
    #[prost(string, tag = "7")]
    pub wallet_id: String,
    #[prost(int32, tag = "8")]
    pub transaction_type: i32,
    #[prost(double, tag = "9")]
    pub asset_amount: f64,
    #[prost(double, tag = "10")]
    pub estimate_price: f64,
    #[prost(string, tag = "11")]
    pub estimate_asset: String,
    #[prost(string, tag = "12")]
    pub trader_id: String,
    #[prost(string, tag = "13")]
    pub blockchain_symbol: String,
    #[prost(double, tag = "14")]
    pub fee_amount: f64,
    #[prost(int32, tag = "15")]
    pub status: i32,
    #[prost(string, tag = "16")]
    pub external_id: String,
    #[prost(int32, tag = "17")]
    pub payment_provider: i32,
}

impl TransactionSbModel {
    fn as_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}

impl GetMySbModelTopicId for TransactionSbModel {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for TransactionSbModel {
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

impl MySbMessageDeserializer for TransactionSbModel {
    type Item = TransactionSbModel;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = TransactionSbModel::from_bytes(src);

        return match result {
            Ok(model) => Ok(model),
            Err(err) => Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}