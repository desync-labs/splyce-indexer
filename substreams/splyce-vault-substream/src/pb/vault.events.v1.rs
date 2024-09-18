// @generated
// TODO: Create seperate proto files for vault and strategy events 
// Priority: Nice to have

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
pub struct StrategyInitEvent {
    #[prost(bytes="vec", tag="1")]
    pub account_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub strategy_type: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub vault: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub underlying_mint: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub underlying_token_acc: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="6")]
    pub underlying_decimals: u32,
    #[prost(uint64, tag="7")]
    pub deposit_limit: u64,
    #[prost(int64, tag="8")]
    pub deposit_period_ends: i64,
    #[prost(int64, tag="9")]
    pub lock_period_ends: i64,
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
    #[prost(bytes="vec", tag="4")]
    pub depositor: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultUpdateDepositLimitEvent {
    #[prost(bytes="vec", tag="1")]
    pub vault_index: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub new_limit: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultWithdrawlEvent {
    #[prost(bytes="vec", tag="1")]
    pub vault_index: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub total_idle: u64,
    #[prost(uint64, tag="3")]
    pub total_share: u64,
    #[prost(uint64, tag="4")]
    pub assets_to_transfer: u64,
    #[prost(uint64, tag="5")]
    pub shares_to_burn: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrategyDepositEvent {
    #[prost(bytes="vec", tag="1")]
    pub account_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(uint64, tag="3")]
    pub total_assets: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrategyWithdrawEvent {
    #[prost(bytes="vec", tag="1")]
    pub account_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(uint64, tag="3")]
    pub total_assets: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultEvent {
    #[prost(string, tag="1000")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="1001")]
    pub block_height: u64,
    #[prost(int64, tag="1002")]
    pub block_timestamp: i64,
    #[prost(oneof="vault_event::Event", tags="1, 2, 3, 4, 5, 6, 7, 8")]
    pub event: ::core::option::Option<vault_event::Event>,
}
/// Nested message and enum types in `VaultEvent`.
pub mod vault_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        VaultInitialize(super::VaultInitEvent),
        #[prost(message, tag="2")]
        StrategyAdd(super::VaultAddStrtegyEvent),
        #[prost(message, tag="3")]
        VaultDeposit(super::VaultDepositEvent),
        #[prost(message, tag="4")]
        Withdrwal(super::VaultWithdrawlEvent),
        #[prost(message, tag="5")]
        UpdateDepositLimit(super::VaultUpdateDepositLimitEvent),
        #[prost(message, tag="6")]
        StrategyInitialize(super::StrategyInitEvent),
        #[prost(message, tag="7")]
        StrategyDeposit(super::StrategyDepositEvent),
        #[prost(message, tag="8")]
        StrategyWithdraw(super::StrategyWithdrawEvent),
    }
}
/// Raw logs from the vault program
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultEventLogs {
    #[prost(bytes="vec", repeated, tag="1")]
    pub logs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag="2")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub block_height: u64,
    #[prost(int64, tag="4")]
    pub block_timestamp: i64,
}
// @@protoc_insertion_point(module)
