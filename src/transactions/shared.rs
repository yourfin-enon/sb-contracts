use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum TransactionTypeSb {
    Deposit = 0,
    Withdrawal = 1,
}

#[derive(Debug, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum TransactionStatusSb {
    Pending = 0,
    Approved = 1,
    Failed = 2,
}

#[derive(Debug, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum PaymentProvider {
    CoinsPaid = 0,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(int64, tag = "2")]
    pub date_micros: i64,
    #[prost(string, tag = "3")]
    pub asset_symbol: String,
    #[prost(string, tag = "4")]
    pub tx_id: String,
    #[prost(string, tag = "5")]
    pub destination_address: String,
    #[prost(string, optional, tag = "6")]
    pub source_address: Option<String>,
    #[prost(string, tag = "7")]
    pub wallet_id: String,
    #[prost(int32, tag = "8")]
    pub transaction_type: i32,
    #[prost(double, tag = "9")]
    pub asset_amount: f64,
    #[prost(double, tag = "10")]
    pub estimate_price: f64,
    #[prost(string, tag = "11")]
    pub estimate_asset: String,
    #[prost(string, tag = "12")]
    pub trader_id: String,
    #[prost(string, tag = "13")]
    pub blockchain_symbol: String,
    #[prost(double, tag = "14")]
    pub fee_amount: f64,
    #[prost(int32, tag = "15")]
    pub status: i32,
    #[prost(string, tag = "16")]
    pub external_id: String,
    #[prost(int32, tag = "17")]
    pub payment_provider: i32,
}