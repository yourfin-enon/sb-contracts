use std::collections::HashMap;
use my_service_bus_abstractions::{GetMySbModelTopicId, SubscriberError};
use my_service_bus_abstractions::publisher::MySbMessageSerializer;
use my_service_bus_abstractions::subscriber::MySbMessageDeserializer;
use crate::transactions::shared::TransactionSbModel;

pub const TOPIC_NAME: &str = "transaction-received";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionReceivedSbEvent {
    #[prost(message, tag = "1")]
    pub transaction: Option<TransactionSbModel>,
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