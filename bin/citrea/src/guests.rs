use std::collections::HashMap;

use citrea_risc0_adapter::Digest;
use lazy_static::lazy_static;
use risc0_binfmt::compute_image_id;
use sov_rollup_interface::spec::SpecId;

lazy_static! {
    /// The following 2 are used as latest guest builds for tests that use mock DA.
    pub(crate) static ref BATCH_PROOF_LATEST_MOCK_GUESTS: HashMap<SpecId, (Digest, Vec<u8>)> = {
        let mut m = HashMap::new();
        let code = include_bytes!("../../../target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/batch_proof_mock/batch_proof_mock").to_vec();
        let id = compute_image_id(&code).unwrap();

        m.insert(SpecId::Genesis, (id, code));
        m
    };
    pub(crate) static ref LIGHT_CLIENT_LATEST_MOCK_GUESTS: HashMap<SpecId, (Digest, Vec<u8>)> = {
        let mut m = HashMap::new();
        let code = include_bytes!("../../../target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/light_client_proof_mock/light_client_proof_mock").to_vec();
        let id = compute_image_id(&code).unwrap();

        m.insert(SpecId::Genesis, (id, code));
        m
    };
    /// The following 2 are used as latest guest builds for tests that use Bitcoin DA.
    pub(crate) static ref BATCH_PROOF_LATEST_BITCOIN_GUESTS: HashMap<SpecId, (Digest, Vec<u8>)> = {
        let mut m = HashMap::new();
        let code = include_bytes!("../../../target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/batch_proof_bitcoin/batch_proof_bitcoin").to_vec();
        let id = compute_image_id(&code).unwrap();

        m.insert(SpecId::Genesis, (id, code));
        m
    };
    pub(crate) static ref LIGHT_CLIENT_LATEST_BITCOIN_GUESTS: HashMap<SpecId, (Digest, Vec<u8>)> = {
        let mut m = HashMap::new();
        let code = include_bytes!("../../../target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/light_client_proof_bitcoin/light_client_proof_bitcoin").to_vec();
        let id = compute_image_id(&code).unwrap();

        m.insert(SpecId::Genesis, (id, code));
        m
    };
    /// Production guests
    pub(crate) static ref BATCH_PROOF_MAINNET_GUESTS: HashMap<SpecId, (Digest, Vec<u8>)> = {
        let mut m = HashMap::new();
        let code = include_bytes!("../../../resources/guests/risc0/mainnet/batch-0.elf").to_vec();
        let id = compute_image_id(&code).unwrap();

        m.insert(SpecId::Genesis, (id, code));
        m
    };
    pub(crate) static ref BATCH_PROOF_TESTNET_GUESTS: HashMap<SpecId, (Digest, Vec<u8>)> = {
        let mut m = HashMap::new();
        let code = include_bytes!("../../../resources/guests/risc0/testnet/batch-0.elf").to_vec();
        let id = compute_image_id(&code).unwrap();
        m.insert(SpecId::Genesis, (id, code));
        m
    };
    pub(crate) static ref LIGHT_CLIENT_MAINNET_GUESTS: HashMap<SpecId, (Digest, Vec<u8>)> = {
        let mut m = HashMap::new();
        let code = include_bytes!("../../../resources/guests/risc0/mainnet/light-0.elf").to_vec();
        let id = compute_image_id(&code).unwrap();
        m.insert(SpecId::Genesis, (id, code));
        m
    };
    pub(crate) static ref LIGHT_CLIENT_TESTNET_GUESTS: HashMap<SpecId, (Digest, Vec<u8>)> = {
        let mut m = HashMap::new();
        let code = include_bytes!("../../../resources/guests/risc0/testnet/light-0.elf").to_vec();
        let id = compute_image_id(&code).unwrap();
        m.insert(SpecId::Genesis, (id, code));
        m
    };
}
