use my_service_bus_abstractions::{
    publisher::MySbMessageSerializer, subscriber::MySbMessageDeserializer, GetMySbModelTopicId,
    SubscriberError,
};
use std::collections::HashMap;
use super::order::OrderSbModel;

pub const TOPIC_NAME: &str = "position-opened-events";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionOpenedSbEvent {
    #[prost(string, tag = "1")]
    pub id: String,
    
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<OrderSbModel>,

    #[prost(int64, tag = "3")]
    pub open_date: i64,

    #[prost(map = "string, double", tag = "4")]
    pub open_asset_prices: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
}

impl PositionOpenedSbEvent {
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

impl GetMySbModelTopicId for PositionOpenedSbEvent {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for PositionOpenedSbEvent {
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

impl MySbMessageDeserializer for PositionOpenedSbEvent {
    type Item = PositionOpenedSbEvent;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = PositionOpenedSbEvent::from_bytes(src);

        match result {
            Ok(model) => return Ok(model),
            Err(err) => return Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}
