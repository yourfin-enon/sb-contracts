use crate::shared::NullableDoubleSb;
use my_service_bus_abstractions::{
    publisher::MySbMessageSerializer, subscriber::MySbMessageDeserializer, GetMySbModelTopicId,
    SubscriberError,
};
use std::collections::HashMap;
use crate::positions::order::OrderSbModel;

pub const TOPIC_NAME: &str = "wallet-position-closed";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionClosedSbEvent {
    #[prost(string, tag = "1")]
    pub id: String,

    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<OrderSbModel>,

    #[prost(message, optional, tag = "3")]
    pub pnl: ::core::option::Option<NullableDoubleSb>,
    
    #[prost(map = "string, double", tag = "4")]
    pub asset_pnls: ::std::collections::HashMap<::prost::alloc::string::String, f64>,

    #[prost(int64, tag = "5")]
    pub close_date: i64,

    #[prost(double, tag = "6")]
    pub close_price: f64,

    #[prost(map = "string, double", tag = "7")]
    pub close_asset_prices: ::std::collections::HashMap<::prost::alloc::string::String, f64>,

    #[prost(enumeration = "PositionCloseReasonSb", tag = "8")]
    pub reason: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionCloseReasonSb {
    ClientCommand = 0,
    StopOut = 1,
    TakeProfit = 2,
    StopLoss = 3,
    AdminCommand = 4,
}

impl PositionClosedSbEvent {
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

impl GetMySbModelTopicId for PositionClosedSbEvent {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for PositionClosedSbEvent {
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

impl MySbMessageDeserializer for PositionClosedSbEvent {
    type Item = PositionClosedSbEvent;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = PositionClosedSbEvent::from_bytes(src);

        match result {
            Ok(model) => return Ok(model),
            Err(err) => return Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}
