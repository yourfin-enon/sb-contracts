use crate::shared::{from_bytes, into_bytes};
use my_service_bus_abstractions::{publisher::MySbMessageSerializer, GetMySbModelTopicId};
use serde::{Deserialize, Serialize};

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
    #[prost(optional, string, tag = "5")]
    pub additional_data_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalVerificationAdditionalDataSbModel {
    pub address: String,
    pub asset_amount: f64,
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
        let content = into_bytes(self);

        match content {
            Ok(content) => Ok((content, headers)),
            Err(err) => Err(format!("{err}")),
        }
    }
}
