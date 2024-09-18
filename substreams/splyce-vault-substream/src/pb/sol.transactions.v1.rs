// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<::substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction>,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(int64, tag="3")]
    pub block_timestamp: i64,
}
// @@protoc_insertion_point(module)
