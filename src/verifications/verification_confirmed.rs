use my_service_bus_abstractions::{publisher::MySbMessageSerializer, GetMySbModelTopicId};

pub const SEND_TOPIC_NAME: &str = "verification-confirmed";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerificationConfirmedSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    #[prost(int32, tag = "3")]
    pub reason: i32,
}

impl VerificationConfirmedSbModel {
    fn as_bytes(&self) -> Result<Vec<u8>, prost::EncodeError> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result)?;
        Ok(result)
    }

    fn _from_bytes(bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        prost::Message::decode(bytes)
    }
}

impl GetMySbModelTopicId for VerificationConfirmedSbModel {
    fn get_topic_id() -> &'static str {
        SEND_TOPIC_NAME
    }
}

impl MySbMessageSerializer for VerificationConfirmedSbModel {
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