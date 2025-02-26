#![no_main]
use citrea_light_client_prover::circuit::run_circuit;
use citrea_risc0_adapter::guest::Risc0Guest;
use sov_mock_da::MockDaVerifier;
use sov_rollup_interface::zk::ZkvmGuest;

risc0_zkvm::guest::entry!(main);

const L2_GENESIS_ROOT: [u8; 32] = match const_hex::const_decode_to_array(b"dacb59b0ff5d16985a8418235133eee37758a3ac1b76ab6d1f87c6df20e4d4da") {
    Ok(root) => root,
    Err(_) => panic!("Can't happen"),
};

const BATCH_PROOF_METHOD_ID: [u32; 8] = citrea_risc0_batch_proof::BATCH_PROOF_MOCK_ID;

const BATCH_PROVER_DA_PUBLIC_KEY: [u8; 33] = match const_hex::const_decode_to_array(b"03eedab888e45f3bdc3ec9918c491c11e5cf7af0a91f38b97fbc1e135ae4056601") {
    Ok(pub_key) => pub_key,
    Err(_) => panic!("Can't happen"),
};

pub fn main() {
    let guest = Risc0Guest::new();

    let da_verifier = MockDaVerifier {};

    let input = guest.read_from_host();

    let output = run_circuit::<MockDaVerifier, Risc0Guest>(da_verifier, input, L2_GENESIS_ROOT, BATCH_PROOF_METHOD_ID, &BATCH_PROVER_DA_PUBLIC_KEY).unwrap();

    guest.commit(&output);
}
