use my_service_bus_abstractions::{publisher::MySbMessageSerializer, GetMySbModelTopicId};

pub const SEND_TOPIC_NAME: &str = "verification-pending";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerificationPendingSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    #[prost(string, tag = "3")]
    pub code: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub reason: i32,
}

impl VerificationPendingSbModel {
    fn as_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    fn _from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}

impl GetMySbModelTopicId for VerificationPendingSbModel {
    fn get_topic_id() -> &'static str {
        SEND_TOPIC_NAME
    }
}

impl MySbMessageSerializer for VerificationPendingSbModel {
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
