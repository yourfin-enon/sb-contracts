use super::order::OrderSbModel;
use my_service_bus_abstractions::{
    publisher::MySbMessageSerializer, subscriber::MySbMessageDeserializer, GetMySbModelTopicId,
    SubscriberError,
};
use std::collections::HashMap;

pub const TOPIC_NAME: &str = "wallet-position-top-up";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionTopUpSbEvent {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(message, optional, tag = "2")]
    pub order: Option<OrderSbModel>,
    #[prost(message, repeated, tag = "3")]
    pub active_top_ups: Vec<ActiveTopUpSbModel>,
    #[prost(message, repeated, tag = "4")]
    pub canceled_top_ups: Vec<CanceledTopUpSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveTopUpSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(int64, tag = "2")]
    pub date_micros: i64,
    #[prost(map = "string, double", tag = "3")]
    pub assets: HashMap<String, f64>,
    #[prost(double, tag = "4")]
    pub instrument_price: f64,
    #[prost(map = "string, double", tag = "5")]
    pub asset_prices: HashMap<String, f64>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanceledTopUpSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(int64, tag = "2")]
    pub date_micros: i64,
    #[prost(map = "string, double", tag = "3")]
    pub assets: HashMap<String, f64>,
    #[prost(double, tag = "4")]
    pub instrument_price: f64,
    #[prost(map = "string, double", tag = "5")]
    pub asset_prices: HashMap<String, f64>,
    #[prost(double, tag = "6")]
    pub cancel_instrument_price: f64,
    #[prost(int64, tag = "7")]
    pub cancel_date_micros: i64,
}

impl PositionTopUpSbEvent {
    pub fn as_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let version: u8 = 0;
        let mut result = vec![version];
        prost::Message::encode(self, &mut result)?;

        Ok(result)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(&bytes[1..])
    }
}

impl GetMySbModelTopicId for PositionTopUpSbEvent {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for PositionTopUpSbEvent {
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

impl MySbMessageDeserializer for PositionTopUpSbEvent {
    type Item = PositionTopUpSbEvent;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = PositionTopUpSbEvent::from_bytes(src);

        return match result {
            Ok(model) => Ok(model),
            Err(err) => Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}
