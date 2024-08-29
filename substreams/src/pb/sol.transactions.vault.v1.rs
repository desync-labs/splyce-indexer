// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vault {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub underlying_mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub underlying_token_acc: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="5")]
    pub underlying_decimals: u32,
    #[prost(uint64, tag="6")]
    pub total_debt: u64,
    #[prost(uint64, tag="7")]
    pub total_shares: u64,
    #[prost(uint64, tag="8")]
    pub deposit_limit: u64,
    #[prost(uint64, tag="9")]
    pub min_user_deposit: u64,
    #[prost(bool, tag="10")]
    pub is_shutdown: bool,
    #[prost(bytes="vec", repeated, tag="11")]
    pub strategies: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vaults {
    #[prost(message, repeated, tag="1")]
    pub vaults: ::prost::alloc::vec::Vec<Vault>,
}
// @@protoc_insertion_point(module)
