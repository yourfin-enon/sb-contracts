use super::order::OrderSbModel;
use my_service_bus_abstractions::{
    publisher::MySbMessageSerializer, subscriber::MySbMessageDeserializer, GetMySbModelTopicId,
    SubscriberError,
};
use std::collections::HashMap;

pub const TOPIC_NAME: &str = "wallet-position-margin-call";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionMarginCallSbEvent {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(message, optional, tag = "2")]
    pub order: Option<OrderSbModel>,
    #[prost(double, tag = "3")]
    pub pnl: f64,
    #[prost(double, tag = "4")]
    pub loss_percent: f64,
}

impl PositionMarginCallSbEvent {
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

impl GetMySbModelTopicId for PositionMarginCallSbEvent {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for PositionMarginCallSbEvent {
    fn serialize(
        &self,
        headers: Option<std::collections::HashMap<String, String>>,
    ) -> Result<(Vec<u8>, Option<std::collections::HashMap<String, String>>), String> {
        let content = self.as_bytes();

        match content {
            Ok(content) => return Ok((content, headers)),
            Err(err) => return Err(format!("{err}")),
        }
    }
}

impl MySbMessageDeserializer for PositionMarginCallSbEvent {
    type Item = PositionMarginCallSbEvent;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = PositionMarginCallSbEvent::from_bytes(src);

        match result {
            Ok(model) => return Ok(model),
            Err(err) => return Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}