service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "client-authentication")]
pub struct TraderAuthenticatedSbEvent {
    #[prost(string, tag = "1")]
    pub trader_id: String,
    #[prost(string, tag = "2")]
    pub brand: String,
    #[prost(string, tag = "3")]
    pub ip: String,
    #[prost(string, tag = "4")]
    pub user_agent: String,
    #[prost(string, tag = "5")]
    pub lang_id: String,
}
