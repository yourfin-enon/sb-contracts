use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum TransactionTypeSb {
    Deposit = 0,
    Withdrawal = 1,
    CryptoBuy = 2,
}

#[derive(Debug, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum TransactionStatusSb {
    Pending = 0,
    Approved = 1,
    Failed = 2,
    Declined = 3,
}

#[derive(Debug, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum PaymentProvider {
    CoinsPaid = 0,
    Unlimit = 1,
    Monetix = 2,
}

/// Nested message and enum types in `TransactionReceivedSbEvent`.
pub mod transaction_sb_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransactionCryptoBuyInfoSb {
        #[prost(string, tag = "1")]
        pub payment_method: String,
        #[prost(string, tag = "2")]
        pub fail_reason: String,
        #[prost(double, tag = "3")]
        pub crypto_asset_amount: f64,
        #[prost(string, tag = "4")]
        pub crypto_asset_symbol: String,
        #[prost(double, tag = "5")]
        pub fiat_asset_amount: f64,
        #[prost(string, tag = "6")]
        pub fiat_asset_symbol: String,
        #[prost(double, tag = "7")]
        pub tap_fee_asset_amount: f64,
        #[prost(string, tag = "8")]
        pub tap_fee_asset_symbol: String,
    }

    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AdditionalInfo {
        #[prost(message, tag = "20")]
        CryptBuyInfo(TransactionCryptoBuyInfoSb),
    }
}
