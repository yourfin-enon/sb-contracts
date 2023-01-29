use crate::shared::NullableDoubleSb;
use super::order::OrderSbModel;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionSbModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub order: ::core::option::Option<OrderSbModel>,
    #[prost(int64, tag = "3")]
    pub activate_date: i64,
    #[prost(message, optional, tag = "4")]
    pub activate_price: ::core::option::Option<NullableDoubleSb>,
    #[prost(int64, tag = "5")]
    pub close_date: i64,
    #[prost(message, optional, tag = "6")]
    pub close_price: ::core::option::Option<NullableDoubleSb>,
    #[prost(message, optional, tag = "7")]
    pub pnl: ::core::option::Option<NullableDoubleSb>,
    #[prost(map = "string, double", tag = "8")]
    pub activate_invest_amounts: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    #[prost(map = "string, double", tag = "9")]
    pub close_invest_amounts: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    #[prost(map = "string, double", tag = "10")]
    pub open_invest_amounts: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    #[prost(int64, tag = "11")]
    pub open_date: i64,
    #[prost(map = "string, double", tag = "12")]
    pub asset_pnls: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
}