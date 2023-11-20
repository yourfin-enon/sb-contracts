service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "personaldata-phone-updated")]
pub struct PersonalDataPhoneUpdatedSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: String,
    #[prost(optional, string, tag = "2")]
    pub old_phone: Option<String>,
    #[prost(string, tag = "3")]
    pub new_phone: String,
}
