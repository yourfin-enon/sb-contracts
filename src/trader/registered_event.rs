service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "client-register")]
pub struct TraderRegisteredSbEvent {
    #[prost(string, tag = "1")]
    pub trader_id: String,
}
