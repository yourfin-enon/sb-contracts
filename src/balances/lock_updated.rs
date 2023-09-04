use std::collections::HashMap;
use my_service_bus_abstractions::{GetMySbModelTopicId, SubscriberError};
use my_service_bus_abstractions::publisher::MySbMessageSerializer;
use my_service_bus_abstractions::subscriber::MySbMessageDeserializer;
use crate::balances::shared::BalanceSbModel;
use crate::shared::{from_bytes, into_bytes};

pub const TOPIC_NAME: &str = "wallet-balance-lock-updated";

pub struct BalanceLockUpdatedSbEvent {
    #[prost(message, tag = "1")]
    pub balances: BalanceSbModel,
}

impl GetMySbModelTopicId for BalanceLockUpdatedSbEvent {
    fn get_topic_id() -> &'static str {
        TOPIC_NAME
    }
}

impl MySbMessageSerializer for BalanceLockUpdatedSbEvent {
    fn serialize(
        &self,
        headers: Option<HashMap<String, String>>,
    ) -> Result<(Vec<u8>, Option<HashMap<String, String>>), String> {
        let content = into_bytes(self);

        match content {
            Ok(content) => Ok((content, headers)),
            Err(err) => Err(format!("{err}")),
        }
    }
}

impl MySbMessageDeserializer for BalanceLockUpdatedSbEvent {
    type Item = BalanceLockUpdatedSbEvent;
    fn deserialize(
        src: &[u8],
        _headers: &Option<HashMap<String, String>>,
    ) -> Result<Self::Item, SubscriberError> {
        let result = from_bytes(src);

        match result {
            Ok(model) => Ok(model),
            Err(err) => Err(SubscriberError::CanNotDeserializeMessage(format!("{err}"))),
        }
    }
}