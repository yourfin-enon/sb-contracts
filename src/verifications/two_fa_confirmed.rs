service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "two-fa-confirmed")]
pub struct TwoFaConfirmedSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: String,
    #[prost(int32, tag = "2")]
    pub action: i32,
    #[prost(string, tag = "3")]
    pub session_id: String,
}
