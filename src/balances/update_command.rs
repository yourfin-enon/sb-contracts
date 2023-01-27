use my_service_bus_abstractions::{
    publisher::MySbMessageSerializer, subscriber::MySbMessageDeserializer, GetMySbModelTopicId,
    SubscriberError,
};
use std::collections::HashMap;

pub const TOPIC_NAME: &str = "update-balances-command";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBalancesSbCommand {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
    #[prost(string, tag = "3")]
    pub operation_type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub updates: ::prost::alloc::vec::Vec<BalanceUpdateSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdateSbModel {
    #[prost(int32, tag = "1")]
    pub number: i32,
    #[prost(string, tag = "2")]
    pub asset_symbol: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub amount: f64,
    #[prost(string, tag = "4")]
    pub wallet_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub trader_id: ::prost::alloc::string::String,
    #[prost(double, tag = "6")]
    pub reserve_amount: f64,
}

impl UpdateBalancesSbCommand {
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

impl GetMySbModelTopicId for UpdateBalancesSbCommand {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for UpdateBalancesSbCommand {
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

impl MySbMessageDeserializer for UpdateBalancesSbCommand {
    type Item = UpdateBalancesSbCommand;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = UpdateBalancesSbCommand::from_bytes(src);

        match result {
            Ok(model) => return Ok(model),
            Err(err) => return Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}