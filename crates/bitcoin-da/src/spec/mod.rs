use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use sov_rollup_interface::da::DaSpec;

use self::address::AddressWrapper;
use self::blob::BlobWithSender;
use self::block_hash::BlockHashWrapper;
use self::header::HeaderWrapper;
use self::proof::InclusionMultiProof;
use self::transaction::TransactionWrapper;

pub mod address;
pub mod blob;
pub mod block;
mod block_hash;
pub mod header;
#[cfg(feature = "native")]
pub mod header_stream;
pub mod proof;
pub mod transaction;
pub mod utxo;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BitcoinSpec;

pub struct RollupParams {
    pub to_light_client_prefix: Vec<u8>,
    pub to_batch_proof_prefix: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitcoinNetwork {
    Mainnet,
    Testnet4,
    Signet,
    Regtest,
}

impl BitcoinNetwork {
    pub fn is_testnet4(&self) -> bool {
        *self == Self::Testnet4
    }

    pub fn is_regtest(&self) -> bool {
        *self == Self::Regtest
    }
}

impl DaSpec for BitcoinSpec {
    type SlotHash = BlockHashWrapper;

    type ChainParams = RollupParams;

    type BlockHeader = HeaderWrapper;

    type BlobTransaction = BlobWithSender;

    type Address = AddressWrapper;

    type InclusionMultiProof = InclusionMultiProof;

    type CompletenessProof = Vec<TransactionWrapper>;

    type Network = BitcoinNetwork;
}
