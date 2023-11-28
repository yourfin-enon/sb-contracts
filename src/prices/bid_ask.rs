use super::bid_ask_date_time::BidAskDateTime;
service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model_with_version(topic_id = "bidask")]
pub struct BidAskSbModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, tag = "2")]
    pub date_time: ::core::option::Option<BidAskDateTime>,
    #[prost(double, tag = "3")]
    pub bid: f64,
    #[prost(double, tag = "4")]
    pub ask: f64,
    #[prost(double, tag = "5")]
    pub bid_volume: f64,
    #[prost(double, tag = "6")]
    pub ask_volume: f64,
}

#[cfg(test)]
mod tests {
    use crate::prices::bid_ask::BidAskSbModel;

    #[test]
    fn ser_der() {
        let bid_ask = BidAskSbModel {
            id: "BTCUSDT".to_string(),
            date_time: None,
            bid: 1.1,
            ask: 0.9,
            bid_volume: 1.0,
            ask_volume: 1.0,
        };

        let bytes = bid_ask.as_protobuf_bytes().unwrap();
        let _bid_ask = BidAskSbModel::from_protobuf_bytes(&bytes);
    }
}