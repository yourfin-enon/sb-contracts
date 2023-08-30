#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdateSbModel {
    #[prost(string, tag = "1")]
    pub trader_id: String,
    #[prost(string, tag = "2")]
    pub wallet_id: String,
    #[prost(int32, tag = "3")]
    pub number: i32,
    #[prost(string, tag = "4")]
    pub asset_symbol: String,
    #[prost(double, tag = "5")]
    pub amount: f64,
    #[prost(double, tag = "6")]
    pub reserve_amount: f64,
    #[prost(int32, tag = "7")]
    pub reason: i32,
    #[prost(string, tag = "8")]
    pub update_id: String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdateInfoSbModel {
    #[prost(string, tag = "1")]
    pub update_id: String,
    #[prost(int32, tag = "2")]
    pub update_status: i32,
    #[prost(string, tag = "3")]
    pub balance_id: String,
    #[prost(double, tag = "4")]
    pub available_amount_before: f64,
    #[prost(double, tag = "5")]
    pub reserved_amount_before: f64,
    #[prost(double, tag = "6")]
    pub available_amount_after: f64,
    #[prost(double, tag = "7")]
    pub reserved_amount_after: f64,
    #[prost(int64, tag = "8")]
    pub update_date_micros: i64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceSbModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub trader_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub created_date_micros: i64,
    #[prost(string, tag = "4")]
    pub wallet_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub asset_symbol: ::prost::alloc::string::String,
    #[prost(double, tag = "6")]
    pub amount: f64,
    #[prost(int64, tag = "7")]
    pub last_update_date_micros: i64,
    #[prost(int32, tag = "8")]
    pub updates_count: i32,
    #[prost(bool, tag = "9")]
    pub is_locked: bool,
    #[prost(double, tag = "10")]
    pub reserved_amount: f64,
}