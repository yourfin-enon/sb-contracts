use std::collections::HashMap;

use my_service_bus_abstractions::{
    publisher::MySbMessageSerializer, subscriber::MySbMessageDeserializer, GetMySbModelTopicId,
    SubscriberError,
};

pub const TOPIC_NAME: &str = "client-session-actions";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientSessionActionSb {
    #[prost(string, tag = "1")]
    pub session_id: String,
    #[prost(string, tag = "2")]
    pub trader_id: String,
    #[prost(string, tag = "3")]
    pub brand_id: String,
    #[prost(int64, tag = "4")]
    pub created_ts: i64,
    #[prost(string, tag = "5")]
    pub device_uuid: String,
    #[prost(string, tag = "6")]
    pub user_agent: String,
    #[prost(string, tag = "7")]
    pub ip: String,
    #[prost(string, tag = "8")]
    pub ip_country: String,
    #[prost(int32, tag = "9")]
    pub action_type: i32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientSessionSb {

}

impl ClientSessionActionSb {
    fn as_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}

impl GetMySbModelTopicId for ClientSessionActionSb {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for ClientSessionActionSb {
    fn serialize(
        &self,
        headers: Option<HashMap<String, String>>,
    ) -> Result<(Vec<u8>, Option<HashMap<String, String>>), String> {
        let content = self.as_bytes();

        return match content {
            Ok(content) => Ok((content, headers)),
            Err(err) => Err(format!("{err}")),
        }
    }
}

impl MySbMessageDeserializer for ClientSessionActionSb {
    type Item = ClientSessionActionSb;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = ClientSessionActionSb::from_bytes(src);

        return match result {
            Ok(model) => Ok(model),
            Err(err) => Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}
