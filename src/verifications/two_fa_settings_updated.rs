service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "two-fa-settings-updated")]
pub struct TwoFaSettingsUpdatedSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: String,
    #[prost(bool, optional, tag = "2")]
    pub google_code_enabled: Option<bool>,
    #[prost(string, tag = "3")]
    pub device: String,
    #[prost(string, tag = "4")]
    pub ip: String,
}
