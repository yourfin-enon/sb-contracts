use crate::shared::{from_bytes, from_bytes_with_version, into_bytes, into_bytes_with_version};
use my_service_bus_abstractions::subscriber::MySbMessageDeserializer;
use my_service_bus_abstractions::{
    publisher::MySbMessageSerializer, GetMySbModelTopicId, SubscriberError,
};
use std::collections::HashMap;

pub const TOPIC_NAME: &str = "client-register";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderRegisterSbEvent {
    #[prost(string, tag = "1")]
    pub trader_id: String,
}

impl GetMySbModelTopicId for TraderRegisterSbEvent {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for TraderRegisterSbEvent {
    fn serialize(
        &self,
        headers: Option<HashMap<String, String>>,
    ) -> Result<(Vec<u8>, Option<HashMap<String, String>>), String> {
        let content = into_bytes_with_version(self);

        match content {
            Ok(content) => Ok((content, headers)),
            Err(err) => Err(format!("{err}")),
        }
    }
}

impl MySbMessageDeserializer for TraderRegisterSbEvent {
    type Item = TraderRegisterSbEvent;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = from_bytes_with_version(src);

        match result {
            Ok(model) => Ok(model),
            Err(err) => Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}
