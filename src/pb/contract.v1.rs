// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub sei_approvals: ::prost::alloc::vec::Vec<SeiApproval>,
    #[prost(message, repeated, tag="2")]
    pub sei_transfers: ::prost::alloc::vec::Vec<SeiTransfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Calls {
    #[prost(message, repeated, tag="1")]
    pub sei_call_approves: ::prost::alloc::vec::Vec<SeiApproveCall>,
    #[prost(message, repeated, tag="2")]
    pub sei_call_transfers: ::prost::alloc::vec::Vec<SeiTransferCall>,
    #[prost(message, repeated, tag="3")]
    pub sei_call_transfer_froms: ::prost::alloc::vec::Vec<SeiTransferFromCall>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventsCalls {
    #[prost(message, optional, tag="1")]
    pub events: ::core::option::Option<Events>,
    #[prost(message, optional, tag="2")]
    pub calls: ::core::option::Option<Calls>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeiApproval {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeiTransfer {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeiApproveCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub value: ::prost::alloc::string::String,
    #[prost(bool, tag="8")]
    pub output_param0: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeiTransferCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub value: ::prost::alloc::string::String,
    #[prost(bool, tag="8")]
    pub output_param0: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeiTransferFromCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub value: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub output_param0: bool,
}
// @@protoc_insertion_point(module)
