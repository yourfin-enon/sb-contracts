#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullableDoubleSb {
    #[prost(double, tag = "1")]
    pub value: f64,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullableBoolSb {
    #[prost(bool, tag = "1")]
    pub value: bool,
}