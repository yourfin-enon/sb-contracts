use crate::shared::{from_bytes, into_bytes};
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
    #[prost(bytes = "vec", tag = "5")]
    pub additional_data_bytes: Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawalVerificationAdditionalDataSbModel {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(double, tag = "2")]
    pub asset_amount: f64,
}

impl VerificationPendingSbModel {
    pub fn get_additional_data<T: ::prost::Message + Default>(
        &self,
    ) -> Result<T, String> {
        let result = from_bytes(&self.additional_data_bytes);

        let Ok(data) = result else {
            return Err(format!("{:?}", result.unwrap_err()));
        };

        Ok(data)
    }

    pub fn set_additional_data<T: ::prost::Message + Default>(
        &mut self,
        data: &T,
    ) -> Result<(), String> {
        let result = into_bytes(data);

        let Ok(data_bytes) = result else {
            return Err(format!("{:?}", result.unwrap_err()));
        };

        self.additional_data_bytes = data_bytes;

        Ok(())
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
        let content = into_bytes(self);

        match content {
            Ok(content) => Ok((content, headers)),
            Err(err) => Err(format!("{err}")),
        }
    }
}
