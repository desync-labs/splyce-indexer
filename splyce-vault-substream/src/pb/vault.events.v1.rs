// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vault {
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub event_instruction_id: ::prost::alloc::vec::Vec<u8>,
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
    pub minimum_total_idle: u64,
    #[prost(uint64, tag="9")]
    pub total_idle: u64,
    #[prost(uint64, tag="10")]
    pub deposit_limit: u64,
    #[prost(uint64, tag="11")]
    pub min_user_deposit: u64,
    #[prost(bool, tag="12")]
    pub is_shutdown: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vaults {
    #[prost(message, repeated, tag="1")]
    pub vaults: ::prost::alloc::vec::Vec<Vault>,
}
// @@protoc_insertion_point(module)
