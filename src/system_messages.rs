service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "send-system-message")]
pub struct SendSystemMessageSb {
    #[prost(message, tag = "1")]
    pub message: ::core::option::Option<SystemMessageSb>,
    #[prost(string, repeated, tag = "2")]
    pub trader_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemMessageSb {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub create_ts: i64,
    #[prost(string, tag = "3")]
    pub created_by: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub auto_send_ts: i64,
}
