service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "email-events")]
pub struct EmailEventSbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,
    #[prost(string, tag = "2")]
    pub event_name: String,
    #[prost(string, tag = "3")]
    pub provider_name: String,
    #[prost(optional, string, tag = "4")]
    pub template_id: Option<String>,
}
