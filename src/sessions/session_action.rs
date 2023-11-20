service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "client-session-actions")]
pub struct ClientSessionActionSb {
    #[prost(string, tag = "1")]
    pub session_id: String,
    #[prost(string, tag = "2")]
    pub trader_id: String,
    #[prost(string, tag = "3")]
    pub brand_id: String,
    #[prost(int64, tag = "4")]
    pub created_ts: i64,
    #[prost(string, tag = "5")]
    pub device_uuid: String,
    #[prost(string, tag = "6")]
    pub user_agent: String,
    #[prost(string, tag = "7")]
    pub ip: String,
    #[prost(string, tag = "8")]
    pub ip_country: String,
    #[prost(int32, tag = "9")]
    pub action_type: i32,
}
