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
    pub order: Option<OrderSbModel>,

    #[prost(double, optional, tag = "3")]
    pub pnl: Option<f64>,
    
    #[prost(map = "string, double", tag = "4")]
    pub asset_pnls: HashMap<String, f64>,

    #[prost(int64, tag = "5")]
    pub close_date_micros: i64,

    #[prost(double, tag = "6")]
    pub close_price: f64,

    #[prost(map = "string, double", tag = "7")]
    pub close_asset_prices: HashMap<String, f64>,

    #[prost(enumeration = "PositionCloseReasonSb", tag = "8")]
    pub reason: i32,

    #[prost(int32, tag = "9")]
    pub status: i32,

    #[prost(double, optional, tag = "10")]
    pub activate_price: Option<f64>,

    #[prost(double, tag = "11")]
    pub total_invest_amount: f64,

    #[prost(map = "string, double", tag = "12")]
    pub invest_bonus_assets: HashMap<String, f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
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
        headers: Option<HashMap<String, String>>,
    ) -> Result<(Vec<u8>, Option<HashMap<String, String>>), String> {
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
