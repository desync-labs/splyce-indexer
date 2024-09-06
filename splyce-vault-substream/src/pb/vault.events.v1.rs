// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultInitEvent {
    #[prost(bytes="vec", tag="1")]
    pub vault_index: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub underlying_mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub underlying_token_acc: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="4")]
    pub underlying_decimals: u32,
    #[prost(uint64, tag="5")]
    pub deposit_limit: u64,
    #[prost(uint64, tag="6")]
    pub min_user_deposit: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultAddStrtegyEvent {
    #[prost(bytes="vec", tag="1")]
    pub vault_index: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub strategy_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub current_debt: u64,
    #[prost(uint64, tag="4")]
    pub max_debt: u64,
    #[prost(uint64, tag="5")]
    pub last_update: u64,
    #[prost(bool, tag="6")]
    pub is_active: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultDepositEvent {
    #[prost(bytes="vec", tag="1")]
    pub vault_index: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(uint64, tag="3")]
    pub share: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultEvent {
    #[prost(oneof="vault_event::Event", tags="1, 2, 3")]
    pub event: ::core::option::Option<vault_event::Event>,
}
/// Nested message and enum types in `VaultEvent`.
pub mod vault_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        Initialize(super::VaultInitEvent),
        #[prost(message, tag="2")]
        StrategyAdd(super::VaultAddStrtegyEvent),
        #[prost(message, tag="3")]
        VaultDeposit(super::VaultDepositEvent),
    }
}
/// Raw logs from the vault program
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultEventLogs {
    #[prost(bytes="vec", repeated, tag="1")]
    pub logs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
// @@protoc_insertion_point(module)
