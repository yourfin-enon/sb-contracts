use std::collections::HashMap;
use my_service_bus_abstractions::{publisher::MySbMessageSerializer, GetMySbModelTopicId, SubscriberError};
use my_service_bus_abstractions::subscriber::MySbMessageDeserializer;

pub const TOPIC_NAME: &str = "personaldata-email-updated";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonalDataEmailUpdatedSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: String,
}

impl PersonalDataEmailUpdatedSbModel {
    fn as_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}

impl GetMySbModelTopicId for PersonalDataEmailUpdatedSbModel {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for PersonalDataEmailUpdatedSbModel {
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

impl MySbMessageDeserializer for PersonalDataEmailUpdatedSbModel {
    type Item = PersonalDataEmailUpdatedSbModel;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = PersonalDataEmailUpdatedSbModel::from_bytes(src);

        return match result {
            Ok(model) => Ok(model),
            Err(err) => Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}