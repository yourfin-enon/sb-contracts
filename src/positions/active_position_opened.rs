use super::order::OrderSbModel;
use my_service_bus_abstractions::{
    publisher::MySbMessageSerializer, subscriber::MySbMessageDeserializer, GetMySbModelTopicId,
    SubscriberError,
};
use std::collections::HashMap;

pub const TOPIC_NAME: &str = "active-position-opened-events";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivePositionOpenedSbEvent {
    #[prost(string, tag = "1")]
    pub id: String,

    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<OrderSbModel>,

    #[prost(double, tag = "3")]
    pub activate_price: f64,

    #[prost(int64, tag = "4")]
    pub activate_date: i64,

    #[prost(map = "string, double", tag = "5")]
    pub activate_invest_amounts: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
}

impl ActivePositionOpenedSbEvent {
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

impl GetMySbModelTopicId for ActivePositionOpenedSbEvent {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for ActivePositionOpenedSbEvent {
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

impl MySbMessageDeserializer for ActivePositionOpenedSbEvent {
    type Item = ActivePositionOpenedSbEvent;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = ActivePositionOpenedSbEvent::from_bytes(src);

        match result {
            Ok(model) => return Ok(model),
            Err(err) => return Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}
