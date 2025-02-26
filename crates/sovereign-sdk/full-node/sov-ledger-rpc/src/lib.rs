#![forbid(unsafe_code)]

use alloy_primitives::U64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;
use sov_rollup_interface::rpc::{
    BatchProofResponse, LastVerifiedBatchProofResponse, SequencerCommitmentResponse,
    SoftConfirmationResponse, SoftConfirmationStatus, VerifiedBatchProofResponse,
};

#[cfg(feature = "server")]
pub mod server;

/// A 32-byte hash [`serde`]-encoded as a hex string optionally prefixed with
/// `0x`. See [`sov_rollup_interface::rpc::utils::rpc_hex`].
#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct HexHash(#[serde(with = "sov_rollup_interface::rpc::utils::rpc_hex")] pub [u8; 32]);

impl From<[u8; 32]> for HexHash {
    fn from(v: [u8; 32]) -> Self {
        Self(v)
    }
}

/// A [`jsonrpsee`] trait for interacting with the ledger JSON-RPC API.
///
/// Client and server implementations are automatically generated by
/// [`jsonrpsee`], see [`LedgerRpcClient`] and [`LedgerRpcServer`].
///
/// For more information about the specific methods, see the
/// [`sov_rollup_interface::rpc`] module.

#[cfg_attr(
    all(feature = "server", feature = "client"),
    rpc(server, client, namespace = "ledger")
)]
#[cfg_attr(
    all(feature = "server", not(feature = "client")),
    rpc(server, namespace = "ledger")
)]
#[cfg_attr(
    all(not(feature = "server"), feature = "client"),
    rpc(client, namespace = "ledger")
)]
pub trait LedgerRpc {
    /// Gets a single soft confirmation by number.
    #[method(name = "getSoftConfirmationByNumber")]
    #[blocking]
    fn get_soft_confirmation_by_number(
        &self,
        number: U64,
    ) -> RpcResult<Option<SoftConfirmationResponse>>;

    /// Gets a single soft confirmation by hash.
    #[method(name = "getSoftConfirmationByHash")]
    #[blocking]
    fn get_soft_confirmation_by_hash(
        &self,
        hash: HexHash,
    ) -> RpcResult<Option<SoftConfirmationResponse>>;

    /// Gets all soft confirmations with numbers `range.start` to `range.end`.
    #[method(name = "getSoftConfirmationRange")]
    #[blocking]
    fn get_soft_confirmation_range(
        &self,
        start: U64,
        end: U64,
    ) -> RpcResult<Vec<Option<SoftConfirmationResponse>>>;

    /// Gets a single event by number.
    #[method(name = "getSoftConfirmationStatus")]
    #[blocking]
    fn get_soft_confirmation_status(
        &self,
        soft_confirmation_receipt: U64,
    ) -> RpcResult<SoftConfirmationStatus>;

    /// Gets the L2 genesis state root.
    #[method(name = "getL2GenesisStateRoot")]
    #[blocking]
    fn get_l2_genesis_state_root(&self) -> RpcResult<Option<Vec<u8>>>;

    /// Gets the commitments in the DA slot with the given height.
    #[method(name = "getSequencerCommitmentsOnSlotByNumber")]
    #[blocking]
    fn get_sequencer_commitments_on_slot_by_number(
        &self,
        height: U64,
    ) -> RpcResult<Option<Vec<SequencerCommitmentResponse>>>;

    /// Gets the commitments in the DA slot with the given hash.
    #[method(name = "getSequencerCommitmentsOnSlotByHash")]
    #[blocking]
    fn get_sequencer_commitments_on_slot_by_hash(
        &self,
        hash: HexHash,
    ) -> RpcResult<Option<Vec<SequencerCommitmentResponse>>>;

    /// Gets proof by slot height.
    #[method(name = "getBatchProofsBySlotHeight")]
    #[blocking]
    fn get_batch_proofs_by_slot_height(
        &self,
        height: U64,
    ) -> RpcResult<Option<Vec<BatchProofResponse>>>;

    /// Gets proof by slot hash.
    #[method(name = "getBatchProofsBySlotHash")]
    #[blocking]
    fn get_batch_proofs_by_slot_hash(
        &self,
        hash: HexHash,
    ) -> RpcResult<Option<Vec<BatchProofResponse>>>;

    /// Gets the height pf most recent committed soft confirmation.
    #[method(name = "getHeadSoftConfirmation")]
    #[blocking]
    fn get_head_soft_confirmation(&self) -> RpcResult<Option<SoftConfirmationResponse>>;

    /// Gets the height pf most recent committed soft confirmation.
    #[method(name = "getHeadSoftConfirmationHeight")]
    #[blocking]
    fn get_head_soft_confirmation_height(&self) -> RpcResult<u64>;

    /// Gets verified proofs by slot height
    #[method(name = "getVerifiedBatchProofsBySlotHeight")]
    #[blocking]
    fn get_verified_batch_proofs_by_slot_height(
        &self,
        height: U64,
    ) -> RpcResult<Option<Vec<VerifiedBatchProofResponse>>>;

    /// Gets last verified proog
    #[method(name = "getLastVerifiedBatchProof")]
    #[blocking]
    fn get_last_verified_batch_proof(&self) -> RpcResult<Option<LastVerifiedBatchProofResponse>>;

    /// Get last scanned l1 height
    #[method(name = "getLastScannedL1Height")]
    #[blocking]
    fn get_last_scanned_l1_height(&self) -> RpcResult<u64>;
}
