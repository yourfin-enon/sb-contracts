use std::collections::HashMap;
use my_service_bus_abstractions::{publisher::MySbMessageSerializer, GetMySbModelTopicId, SubscriberError};
use my_service_bus_abstractions::subscriber::MySbMessageDeserializer;

pub const TOPIC_NAME: &str = "two-fa-confirmed";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwoFaConfirmedSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: String,
    #[prost(int32, tag = "2")]
    pub reason: i32,
    #[prost(string, tag = "3")]
    pub session_id: String,
}

impl TwoFaConfirmedSbModel {
    fn as_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}

impl GetMySbModelTopicId for TwoFaConfirmedSbModel {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for TwoFaConfirmedSbModel {
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

impl MySbMessageDeserializer for TwoFaConfirmedSbModel {
    type Item = TwoFaConfirmedSbModel;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = TwoFaConfirmedSbModel::from_bytes(src);

        return match result {
            Ok(model) => Ok(model),
            Err(err) => Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}
