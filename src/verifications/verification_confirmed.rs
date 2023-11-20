service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "verification-confirmed")]
pub struct VerificationConfirmedSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    #[prost(int32, tag = "3")]
    pub reason: i32,
    #[prost(bool, tag = "4")]
    pub has_prev: bool,
}
