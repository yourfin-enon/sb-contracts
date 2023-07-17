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
}
