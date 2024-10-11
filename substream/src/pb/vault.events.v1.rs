// @generated
// TODO: Create seperate proto files for vault and strategy events 
// Priority: Nice to have

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultInitEvent {
    #[prost(string, tag="1")]
    pub vault_index: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub underlying_mint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub underlying_token_acc: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub underlying_decimals: u32,
    #[prost(string, tag="5")]
    pub share_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub share_token_acc: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub share_decimals: u32,
    #[prost(uint64, tag="8")]
    pub deposit_limit: u64,
    #[prost(uint64, tag="9")]
    pub min_user_deposit: u64,
    #[prost(uint64, tag="10")]
    pub performance_fee: u64,
    #[prost(string, tag="11")]
    pub vault_pda: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrategyInitEvent {
    #[prost(string, tag="1")]
    pub account_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub strategy_type: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub vault: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub underlying_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub underlying_token_acc: ::prost::alloc::string::String,
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
pub struct VaultAddStrategyEvent {
    #[prost(string, tag="1")]
    pub vault_index: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub strategy_key: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub current_debt: u64,
    #[prost(uint64, tag="4")]
    pub max_debt: u64,
    #[prost(int64, tag="5")]
    pub last_update: i64,
    #[prost(bool, tag="6")]
    pub is_active: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultDepositEvent {
    #[prost(string, tag="1")]
    pub vault_index: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub total_debt: u64,
    #[prost(uint64, tag="3")]
    pub total_idle: u64,
    #[prost(uint64, tag="4")]
    pub total_share: u64,
    #[prost(uint64, tag="5")]
    pub amount: u64,
    #[prost(uint64, tag="6")]
    pub share: u64,
    #[prost(string, tag="7")]
    pub token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub share_account: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub authority: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultUpdateDepositLimitEvent {
    #[prost(string, tag="1")]
    pub vault_index: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub new_limit: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VaultWithdrawlEvent {
    #[prost(string, tag="1")]
    pub vault_index: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub total_idle: u64,
    #[prost(uint64, tag="3")]
    pub total_share: u64,
    #[prost(uint64, tag="4")]
    pub assets_to_transfer: u64,
    #[prost(uint64, tag="5")]
    pub shares_to_burn: u64,
    #[prost(string, tag="6")]
    pub token_account: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub share_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub authority: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrategyDepositEvent {
    #[prost(string, tag="1")]
    pub account_key: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(uint64, tag="3")]
    pub total_assets: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrategyWithdrawEvent {
    #[prost(string, tag="1")]
    pub account_key: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(uint64, tag="3")]
    pub total_assets: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPerformanceFeeEvent {
    #[prost(string, tag="1")]
    pub account_key: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub fee: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatedCurrentDebtForStrategyEvent {
    #[prost(string, tag="1")]
    pub vault_index: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub strategy_key: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub total_idle: u64,
    #[prost(uint64, tag="4")]
    pub total_debt: u64,
    #[prost(uint64, tag="5")]
    pub new_debt: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrategyReportedEvent {
    #[prost(string, tag="1")]
    pub strategy_key: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub gain: u64,
    #[prost(uint64, tag="3")]
    pub loss: u64,
    #[prost(uint64, tag="4")]
    pub current_debt: u64,
    #[prost(uint64, tag="5")]
    pub protocol_fees: u64,
    #[prost(uint64, tag="6")]
    pub total_fees: u64,
    #[prost(int64, tag="7")]
    pub timestamp: i64,
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
    #[prost(oneof="vault_event::Event", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
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
        StrategyAdd(super::VaultAddStrategyEvent),
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
        #[prost(message, tag="9")]
        UpdatedDebtForStrategy(super::UpdatedCurrentDebtForStrategyEvent),
        #[prost(message, tag="10")]
        StrategyReported(super::StrategyReportedEvent),
        #[prost(message, tag="11")]
        SetPerformanceFee(super::SetPerformanceFeeEvent),
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
