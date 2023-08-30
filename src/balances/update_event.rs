use my_service_bus_abstractions::{
    publisher::MySbMessageSerializer, subscriber::MySbMessageDeserializer, GetMySbModelTopicId,
    SubscriberError,
};
use std::collections::HashMap;
use crate::balances::shared::{BalanceSbModel, BalanceUpdateInfoSbModel, BalanceUpdateSbModel};

pub const TOPIC_NAME: &str = "wallet-balances-updated";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalancesUpdatedSbEvent {
    #[prost(string, tag = "1")]
    pub operation_id: String,
    #[prost(int64, tag = "2")]
    pub date_micros: i64,
    #[prost(string, tag = "3")]
    pub description: String,
    #[prost(message, repeated, tag = "4")]
    pub balances: ::prost::alloc::vec::Vec<BalanceSbModel>,
    #[prost(int32, tag = "5")]
    pub operation_type: i32,
    #[prost(message, repeated, tag = "6")]
    pub updates: ::prost::alloc::vec::Vec<BalanceUpdateSbModel>,
    #[prost(message, repeated, tag = "7")]
    pub update_infos: ::prost::alloc::vec::Vec<BalanceUpdateInfoSbModel>,
}

impl BalancesUpdatedSbEvent {
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

impl GetMySbModelTopicId for BalancesUpdatedSbEvent {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for BalancesUpdatedSbEvent {
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

impl MySbMessageDeserializer for BalancesUpdatedSbEvent {
    type Item = BalancesUpdatedSbEvent;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = BalancesUpdatedSbEvent::from_bytes(src);

        match result {
            Ok(model) => return Ok(model),
            Err(err) => return Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}
