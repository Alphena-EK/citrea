//! Defines the traits that must be implemented by zkVMs. A zkVM like Risc0 consists of two components,
//! a "guest" and a "host". The guest is the zkVM program itself, and the host is the physical machine on
//! which the zkVM is running. Both the guest and the host are required to implement the [`Zkvm`] trait, in
//! addition to the specialized [`ZkvmGuest`] and [`ZkvmHost`] trait which is appropriate to that environment.
//!
//! For a detailed example showing how to implement these traits, see the
//! [risc0 adapter](https://github.com/Sovereign-Labs/sovereign-sdk/tree/main/adapters/risc0)
//! maintained by the Sovereign Labs team.

extern crate alloc;

use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;
use core::convert::Into;
use core::fmt::Debug;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::da::DaSpec;
use crate::soft_confirmation::SignedSoftConfirmation;

/// The ZK proof generated by the [`ZkvmHost::run`] method.
pub type Proof = Vec<u8>;

/// A trait implemented by the prover ("host") of a zkVM program.
pub trait ZkvmHost: Zkvm + Clone {
    /// The associated guest type
    type Guest: ZkvmGuest;
    /// Give the guest a piece of advice non-deterministically
    /// `item` is a borsh serialized input to the guest
    fn add_hint(&mut self, item: Vec<u8>);

    /// Simulate running the guest using the provided hints.
    ///
    /// Provides a simulated version of the guest which can be
    /// accessed in the current process.
    fn simulate_with_hints(&mut self) -> Self::Guest;

    /// Run the guest in the true zk environment using the provided hints.
    ///
    /// This runs the guest binary compiled for the zkVM target, optionally
    /// creating a SNARK of correct execution. Running the true guest binary comes
    /// with some mild performance overhead and is not as easy to debug as [`simulate_with_hints`](ZkvmHost::simulate_with_hints).
    fn run(&mut self, elf: Vec<u8>, with_proof: bool) -> Result<Proof, anyhow::Error>;

    /// Extracts public input and receipt from the proof.
    fn extract_output<Da: DaSpec, T: BorshDeserialize>(proof: &Proof) -> Result<T, Self::Error>;

    /// Host recovers pending proving sessions and returns proving results
    fn recover_proving_sessions(&self) -> Result<Vec<Proof>, anyhow::Error>;

    /// Host adds an assumption to the proving session
    /// Assumptions are used for recursive proving
    fn add_assumption(&mut self, receipt_buf: Vec<u8>);
}

/// A Zk proof system capable of proving and verifying arbitrary Rust code
/// Must support recursive proofs.
pub trait Zkvm: Send + Sync {
    /// A commitment to the zkVM program which is being proven
    type CodeCommitment: Clone
        + Debug
        + Serialize
        + DeserializeOwned
        + From<[u32; 8]>
        + Into<[u32; 8]>
        + Send
        + Sync
        + 'static;

    /// The error type which is returned when a proof fails to verify
    type Error: Debug;

    /// Interpret a sequence of a bytes as a proof and attempt to verify it against the code commitment.
    /// If the proof is valid, return a reference to the public outputs of the proof.
    fn verify(
        serialized_proof: &[u8],
        code_commitment: &Self::CodeCommitment,
    ) -> Result<Vec<u8>, Self::Error>;

    /// Extracts the raw output without doing any verification.
    fn extract_raw_output(serialized_proof: &[u8]) -> Result<Vec<u8>, Self::Error>;

    /// Same as [`verify`](Zkvm::verify), except that instead of returning the output
    /// as a serialized array, it returns a state transition structure.
    /// TODO: specify a deserializer for the output
    fn verify_and_extract_output<T: BorshDeserialize>(
        serialized_proof: &[u8],
        code_commitment: &Self::CodeCommitment,
    ) -> Result<T, Self::Error>;
}

/// A trait which is accessible from within a zkVM program.
pub trait ZkvmGuest: Zkvm + Send + Sync {
    /// Obtain "advice" non-deterministically from the host
    fn read_from_host<T: BorshDeserialize>(&self) -> T;
    /// Add a public output to the zkVM proof
    fn commit<T: BorshSerialize>(&self, item: &T);
}

/// State diff produced by the Zk proof
pub type CumulativeStateDiff = BTreeMap<Vec<u8>, Option<Vec<u8>>>;

/// The public output of a SNARK batch proof in Sovereign, this struct makes a claim that
/// the state of the rollup has transitioned from `initial_state_root` to `final_state_root`
///
/// The period of time covered by a state transition proof is a range of L2 blocks whose sequencer
/// commitments are included in the DA slot with hash `da_slot_hash`. The range is inclusive.
#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub struct BatchProofCircuitOutput<Da: DaSpec, Root> {
    /// The state of the rollup before the transition
    pub initial_state_root: Root,
    /// The state of the rollup after the transition
    pub final_state_root: Root,
    /// The hash of the last soft confirmation before the state transition
    pub prev_soft_confirmation_hash: [u8; 32],
    /// The hash of the last soft confirmation in the state transition
    pub final_soft_confirmation_hash: [u8; 32],
    /// State diff of L2 blocks in the processed sequencer commitments.
    pub state_diff: CumulativeStateDiff,
    /// The DA slot hash that the sequencer commitments causing this state transition were found in.
    pub da_slot_hash: Da::SlotHash,
    /// The range of sequencer commitments in the DA slot that were processed.
    /// The range is inclusive.
    pub sequencer_commitments_range: (u32, u32),
    /// Sequencer public key.
    pub sequencer_public_key: Vec<u8>,
    /// Sequencer DA public key.
    pub sequencer_da_public_key: Vec<u8>,
    /// The last processed l2 height in the processed sequencer commitments.
    pub last_l2_height: u64,
    /// Pre-proven commitments L2 ranges which also exist in the current L1 `da_data`.
    pub preproven_commitments: Vec<usize>,
}

/// A trait expressing that two items of a type are (potentially fuzzy) matches.
/// We need a custom trait instead of relying on [`PartialEq`] because we allow fuzzy matches.
pub trait Matches<T> {
    /// Check if two items are a match
    fn matches(&self, other: &T) -> bool;
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
// Prevent serde from generating spurious trait bounds. The correct serde bounds are already enforced by the
// StateTransitionFunction, DA, and Zkvm traits.
#[serde(
    bound = "StateRoot: Serialize + DeserializeOwned, Witness: Serialize + DeserializeOwned, Tx: Serialize + DeserializeOwned"
)]
/// Data required to verify a state transition.
pub struct BatchProofCircuitInput<'txs, StateRoot, Witness, Da: DaSpec, Tx: Clone> {
    /// The state root before the state transition
    pub initial_state_root: StateRoot,
    /// The state root after the state transition
    pub final_state_root: StateRoot,
    /// The hash before the state transition
    pub prev_soft_confirmation_hash: [u8; 32],
    /// The `crate::da::DaData` that are being processed as blobs. Everything that's not `crate::da::DaData::SequencerCommitment` will be ignored.
    pub da_data: Vec<Da::BlobTransaction>,
    /// DA block header that the sequencer commitments were found in.
    pub da_block_header_of_commitments: Da::BlockHeader,
    /// The inclusion proof for all DA data.
    pub inclusion_proof: Da::InclusionMultiProof,
    /// The completeness proof for all DA data.
    pub completeness_proof: Da::CompletenessProof,
    /// Pre-proven commitments L2 ranges which also exist in the current L1 `da_data`.
    pub preproven_commitments: Vec<usize>,
    /// The soft confirmations that are inside the sequencer commitments.
    pub soft_confirmations: VecDeque<Vec<SignedSoftConfirmation<'txs, Tx>>>,
    /// Corresponding witness for the soft confirmations.
    pub state_transition_witnesses: VecDeque<Vec<(Witness, Witness)>>,
    /// DA block headers the soft confirmations was constructed on.
    pub da_block_headers_of_soft_confirmations: VecDeque<Vec<Da::BlockHeader>>,
    /// Sequencer soft confirmation public key.
    pub sequencer_public_key: Vec<u8>,
    /// Sequencer DA public_key: Vec<u8>,
    pub sequencer_da_public_key: Vec<u8>,
    /// The range of sequencer commitments that are being processed.
    /// The range is inclusive.
    pub sequencer_commitments_range: (u32, u32),
}

/// The batch proof that was not verified in the light client circuit because it was missing another proof for state root chaining
/// This struct is passed as an output to the light client circuit
/// After that the new circuit will read that info to update the state root if possible
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, PartialEq, Serialize, Deserialize)]
pub struct BatchProofInfo {
    /// Initial state root of the batch proof
    pub initial_state_root: [u8; 32],
    /// Final state root of the batch proof
    pub final_state_root: [u8; 32],
    /// The last processed l2 height in the batch proof
    pub last_l2_height: u64,
}

impl BatchProofInfo {
    /// Create a new `BatchProofInfo` instance.
    pub fn new(
        initial_state_root: [u8; 32],
        final_state_root: [u8; 32],
        last_l2_height: u64,
    ) -> Self {
        Self {
            initial_state_root,
            final_state_root,
            last_l2_height,
        }
    }
}

/// The output of light client proof
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct LightClientCircuitOutput<Da: DaSpec> {
    /// State root of the node after the light client proof
    pub state_root: [u8; 32],
    /// The method id of the light client proof
    /// This is used to compare the previous light client proof method id with the input (current) method id
    pub light_client_proof_method_id: [u32; 8],
    /// Proved DA block's header hash
    /// This is used to compare the previous DA block hash with first batch proof's DA block hash
    pub da_block_hash: Da::SlotHash,
    /// Height of the blockchain
    pub da_block_height: u64,
    /// Total work done in the DA blockchain
    pub da_total_work: [u8; 32],
    /// Current target bits of DA
    pub da_current_target_bits: u32,
    /// The time of the first block in the current epoch (the difficulty adjustment timestamp)
    pub da_epoch_start_time: u32,
    /// The UNIX timestamps in seconds of the previous 11 blocks
    pub da_prev_11_timestamps: [u32; 11],
    /// Batch proof info from current or previous light client proofs that were not changed and unable to update the state root yet
    pub unchained_batch_proofs_info: Vec<BatchProofInfo>,
    /// Last l2 height the light client proof verifies
    pub last_l2_height: u64,
    /// Genesis state root of Citrea
    pub l2_genesis_state_root: [u8; 32],
    /// A map from tx hash to chunk data
    pub wtxid_data: BTreeMap<[u8; 32], Vec<u8>>,
}

/// The input of light client proof
#[derive(BorshDeserialize, BorshSerialize)]
pub struct LightClientCircuitInput<Da: DaSpec> {
    /// The `crate::da::DaData` that are being processed as blobs.
    pub da_data: Vec<Da::BlobTransaction>,
    /// The inclusion proof for all DA data.
    pub inclusion_proof: Da::InclusionMultiProof,
    /// The completeness proof for all DA data.
    pub completeness_proof: Da::CompletenessProof,
    /// DA block header that the batch proofs were found in.
    pub da_block_header: Da::BlockHeader,

    /// Public key of the batch prover
    pub batch_prover_da_pub_key: Vec<u8>,
    /// Batch proof method id
    pub batch_proof_method_id: [u32; 8],
    /// Light client proof method id
    pub light_client_proof_method_id: [u32; 8],
    /// Light client proof output
    /// Optional because the first light client proof doesn't have a previous proof
    pub previous_light_client_proof_journal: Option<Vec<u8>>,
    /// L2 Genesis state root
    pub l2_genesis_state_root: Option<[u8; 32]>,
    /// A map from tx hash to chunk data
    pub wtxid_data: BTreeMap<[u8; 32], Vec<u8>>,
}
