use my_service_bus_abstractions::{
    publisher::{MySbMessageSerializer},
    GetMySbModelTopicId,
};

pub const SEND_TOPIC_NAME: &str = "send-system-message";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendSystemMessageSb {
    #[prost(message, tag = "1")]
    pub message: ::core::option::Option<SystemMessageSb>,
    #[prost(string, repeated, tag = "2")]
    pub trader_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemMessageSb {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub create_ts: i64,
    #[prost(string, tag = "3")]
    pub created_by: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub auto_send_ts: i64,
}

impl SendSystemMessageSb {
    fn as_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    fn _from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}

impl GetMySbModelTopicId for SendSystemMessageSb {
    fn get_topic_id() -> &'static str {
        SEND_TOPIC_NAME
    }
}

impl MySbMessageSerializer for SendSystemMessageSb {
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
