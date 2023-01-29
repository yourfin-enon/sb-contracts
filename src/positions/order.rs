use crate::shared::NullableDoubleSb;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderSbModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub wallet_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub instrument: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub base_asset: ::prost::alloc::string::String,
    #[prost(double, tag = "5")]
    pub leverage: f64,
    #[prost(enumeration = "OrderSideSb", tag = "7")]
    pub side: i32,
    #[prost(message, optional, tag = "8")]
    pub take_profit: ::core::option::Option<AutoCloseConfigSbModel>,
    #[prost(message, optional, tag = "9")]
    pub stop_loss: ::core::option::Option<AutoCloseConfigSbModel>,
    #[prost(double, tag = "10")]
    pub stop_out_percent: f64,
    #[prost(double, tag = "11")]
    pub margin_call_percent: f64,
    #[prost(map = "string, double", tag = "12")]
    pub invest_assets: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    #[prost(message, optional, tag = "13")]
    pub desire_price: ::core::option::Option<NullableDoubleSb>,
    #[prost(bool, tag = "14")]
    pub top_up_enabled: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoCloseConfigSbModel {
    #[prost(double, tag = "1")]
    pub value: f64,
    #[prost(enumeration = "AutoCloseUnitSb", tag = "2")]
    pub unit: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderSideSb {
    Buy = 0,
    Sell = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AutoCloseUnitSb {
    AssetAmount = 0,
    PriceRate = 1,
}