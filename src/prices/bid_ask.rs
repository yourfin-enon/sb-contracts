use std::collections::HashMap;

use my_service_bus_abstractions::{publisher::MySbMessageSerializer, GetMySbModelTopicId, subscriber::MySbMessageDeserializer, SubscriberError};

use super::bid_ask_date_time::BidAskDateTime;

pub const SEND_TOPIC_NAME: &str = "bidask";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidAskSbModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, tag = "2")]
    pub date_time: ::core::option::Option<BidAskDateTime>,
    #[prost(double, tag = "3")]
    pub bid: f64,
    #[prost(double, tag = "4")]
    pub ask: f64,
}

impl BidAskSbModel {
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

impl GetMySbModelTopicId for BidAskSbModel {
    fn get_topic_id() -> &'static str {
        SEND_TOPIC_NAME
    }
}

impl MySbMessageSerializer for BidAskSbModel {
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

impl MySbMessageDeserializer for BidAskSbModel {
    type Item = BidAskSbModel;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = BidAskSbModel::from_bytes(src);

        match result {
            Ok(model) => return Ok(model),
            Err(err) => return Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}