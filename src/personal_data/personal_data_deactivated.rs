service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "personal-data-deactivated")]
pub struct PersonalDataDeactivatedSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: String,
}
