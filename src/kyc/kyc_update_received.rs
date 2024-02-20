service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "kyc-update-received")]
pub struct KycUpdateReceivedSbModel {
    #[prost(string, tag = "1")]
    pub client_id: String,
    #[prost(string, tag = "2")]
    pub external_id: String,
    #[prost(int32, tag = "3")]
    pub status: i32,
    #[prost(string, tag = "4")]
    pub level_name: String,
}


pub enum KycStatusSb {
    New = 0,
    Processing = 1,
    Declined = 2,
    Rejected = 3,
    Approved = 4,
}
