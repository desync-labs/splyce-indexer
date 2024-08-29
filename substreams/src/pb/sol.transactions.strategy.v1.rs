// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Strategy {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub vault: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub underlying_mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub underlying_token_acc: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="5")]
    pub underlying_decimals: u32,
    #[prost(uint64, tag="6")]
    pub total_funds: u64,
    #[prost(uint64, tag="7")]
    pub total_shares: u64,
    #[prost(uint64, tag="8")]
    pub deposit_limit: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Strategies {
    #[prost(message, repeated, tag="1")]
    pub strategies: ::prost::alloc::vec::Vec<Strategy>,
}
// @@protoc_insertion_point(module)
