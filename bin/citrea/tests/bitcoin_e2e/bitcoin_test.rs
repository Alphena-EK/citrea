use std::time::Duration;

use anyhow::bail;
use async_trait::async_trait;
use bitcoin_da::monitoring::TxStatus;
use bitcoin_da::rpc::DaRpcClient;
use bitcoin_da::service::FINALITY_DEPTH;
use bitcoincore_rpc::RpcApi;
use citrea_e2e::config::TestCaseConfig;
use citrea_e2e::framework::TestFramework;
use citrea_e2e::test_case::{TestCase, TestCaseRunner};
use citrea_e2e::traits::Restart;
use citrea_e2e::Result;
use sov_ledger_rpc::client::RpcClient;
use tokio::time::sleep;

use super::get_citrea_path;

struct BitcoinReorgTest;

#[async_trait]
impl TestCase for BitcoinReorgTest {
    fn test_config() -> TestCaseConfig {
        TestCaseConfig {
            with_sequencer: true,
            with_batch_prover: true,
            n_nodes: 2,
            ..Default::default()
        }
    }

    async fn run_test(&mut self, f: &mut TestFramework) -> Result<()> {
        let (Some(da0), Some(da1)) = (f.bitcoin_nodes.get(0), f.bitcoin_nodes.get(1)) else {
            bail!("Bitcoin nodes not running. Test requires two DA nodes")
        };

        let sequencer = f.sequencer.as_ref().unwrap();
        let batch_prover = f.batch_prover.as_ref().unwrap();

        let min_soft_confirmations_per_commitment =
            sequencer.min_soft_confirmations_per_commitment();

        // Disconnect nodes before generating commitment
        f.bitcoin_nodes.disconnect_nodes().await?;

        for _ in 0..min_soft_confirmations_per_commitment {
            sequencer.client.send_publish_batch_request().await?;
        }

        sequencer
            .wait_for_l2_height(min_soft_confirmations_per_commitment, None)
            .await?;

        // Wait for the sequencer commitments to hit the mempool
        da0.wait_mempool_len(2, None).await?;

        let mempool0 = da0.get_raw_mempool().await?;
        assert_eq!(mempool0.len(), 2);
        let mempool1 = da1.get_raw_mempool().await?;
        assert_eq!(mempool1.len(), 0);

        // Mine block with the sequencer commitment on the main chain
        da0.generate(1, None).await?;

        let original_chain_height = da0.get_block_count().await?;
        let original_chain_hash = da0.get_block_hash(original_chain_height).await?;
        let block = da0.get_block(&original_chain_hash).await?;
        assert_eq!(block.txdata.len(), 3); // Coinbase + seq commit/reveal txs

        let da1_generated_blocks = 2;
        da1.generate(da1_generated_blocks, None).await?;

        // Reconnect nodes and wait for sync
        f.bitcoin_nodes.connect_nodes().await?;
        f.bitcoin_nodes.wait_for_sync(None).await?;

        // Assert that re-org occured
        let new_hash = da0.get_block_hash(original_chain_height).await?;
        assert_ne!(original_chain_hash, new_hash, "Re-org did not occur");

        let mempool0 = da0.get_raw_mempool().await?;
        assert_eq!(mempool0.len(), 2);

        let pending_txs = sequencer
            .client
            .http_client()
            .da_get_pending_transactions()
            .await?;

        assert!(mempool0.contains(&pending_txs[0].txid));
        assert!(mempool0.contains(&pending_txs[1].txid));

        let tx_status = sequencer
            .client
            .http_client()
            .da_get_tx_status(mempool0[0])
            .await?;
        assert!(matches!(tx_status, Some(TxStatus::Pending { .. })));

        // Wait for re-org monitoring
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Seq TXs should be rebroadcasted after re-org
        let mempool1 = da1.get_raw_mempool().await?;
        assert_eq!(mempool1.len(), 2);

        da1.generate(1, None).await?;
        let height = da0.get_block_count().await?;
        let hash = da0.get_block_hash(height).await?;
        let block = da0.get_block(&hash).await?;
        assert_eq!(block.txdata.len(), 3); // Coinbase + seq commit/reveal txs

        da1.generate(FINALITY_DEPTH - 1, None).await?;
        let finalized_height = da1.get_finalized_height().await?;

        batch_prover
            .wait_for_l1_height(finalized_height, None)
            .await?;

        // Generate on da1 and wait for da0 to be back in sync
        f.bitcoin_nodes.wait_for_sync(None).await?;

        // Verify that commitments are included
        let original_commitments = batch_prover
            .client
            .http_client()
            .get_sequencer_commitments_on_slot_by_number(finalized_height)
            .await?
            .unwrap_or_default();

        assert_eq!(original_commitments.len(), 1);

        Ok(())
    }
}

#[tokio::test]
async fn test_bitcoin_reorg() -> Result<()> {
    TestCaseRunner::new(BitcoinReorgTest)
        .set_citrea_path(get_citrea_path())
        .run()
        .await
}

struct DaMonitoringTest;

#[async_trait]
impl TestCase for DaMonitoringTest {
    async fn run_test(&mut self, f: &mut TestFramework) -> Result<()> {
        let da = f.bitcoin_nodes.get(0).unwrap();
        let sequencer = f.sequencer.as_mut().unwrap();

        let min_soft_confirmations_per_commitment =
            sequencer.min_soft_confirmations_per_commitment();

        for _ in 0..min_soft_confirmations_per_commitment {
            sequencer.client.send_publish_batch_request().await?;
        }

        // Wait for the sequencer commitments to hit the mempool
        da.wait_mempool_len(2, None).await?;

        let mempool0 = da.get_raw_mempool().await?;
        assert_eq!(mempool0.len(), 2);

        sleep(Duration::from_secs(1)).await;
        let pending_txs = sequencer
            .client
            .http_client()
            .da_get_pending_transactions()
            .await?;

        assert!(mempool0.contains(&pending_txs[0].txid));
        assert!(mempool0.contains(&pending_txs[1].txid));

        let tx_status = sequencer
            .client
            .http_client()
            .da_get_tx_status(mempool0[0])
            .await?;
        assert!(matches!(tx_status, Some(TxStatus::Pending { .. })));

        da.generate(1, None).await?;

        sleep(Duration::from_secs(1)).await;
        let tx_status = sequencer
            .client
            .http_client()
            .da_get_tx_status(mempool0[0])
            .await?;
        assert!(matches!(tx_status, Some(TxStatus::Confirmed { .. })));

        da.generate(FINALITY_DEPTH, None).await?;

        sleep(Duration::from_secs(1)).await;
        let tx_status = sequencer
            .client
            .http_client()
            .da_get_tx_status(mempool0[0])
            .await?;
        assert!(matches!(tx_status, Some(TxStatus::Finalized { .. })));

        for _ in 0..min_soft_confirmations_per_commitment {
            sequencer.client.send_publish_batch_request().await?;
        }

        // Wait for the sequencer commitments to hit the mempool
        da.wait_mempool_len(2, None).await?;
        let mempool0 = da.get_raw_mempool().await?;

        // Assert that txs are properly monitored
        let pending_txs = sequencer
            .client
            .http_client()
            .da_get_pending_transactions()
            .await?;

        assert!(mempool0.contains(&pending_txs[0].txid));
        assert!(mempool0.contains(&pending_txs[1].txid));

        sequencer.restart(None).await?;

        // Assert that txs are properly monitored after a restart
        let pending_txs = sequencer
            .client
            .http_client()
            .da_get_pending_transactions()
            .await?;

        assert!(mempool0.contains(&pending_txs[0].txid));
        assert!(mempool0.contains(&pending_txs[1].txid));

        Ok(())
    }
}

#[tokio::test]
async fn test_da_monitoring() -> Result<()> {
    TestCaseRunner::new(DaMonitoringTest)
        .set_citrea_path(get_citrea_path())
        .run()
        .await
}

struct CpfpFeeBumpingTest;

#[async_trait]
impl TestCase for CpfpFeeBumpingTest {
    fn test_config() -> TestCaseConfig {
        TestCaseConfig {
            with_sequencer: true,
            with_batch_prover: true,
            ..Default::default()
        }
    }

    async fn run_test(&mut self, f: &mut TestFramework) -> Result<()> {
        let sequencer = f.sequencer.as_mut().unwrap();
        let batch_prover = f.batch_prover.as_mut().unwrap();
        let da = f.bitcoin_nodes.get(0).unwrap();

        // Generate seqcommitments
        for _ in 0..sequencer.min_soft_confirmations_per_commitment() {
            sequencer.client.send_publish_batch_request().await?;
        }

        // Wait for seqcommitments txs to hit mempool
        da.wait_mempool_len(2, None).await?;

        // Get the last txid in chain
        let reveal_tx = sequencer
            .client
            .http_client()
            .da_get_last_monitored_tx()
            .await?
            .unwrap();
        let parent_txid = &reveal_tx.txid;
        let parent_base_fee = reveal_tx.base_fee.unwrap();
        let parent_fee_rate = parent_base_fee as f64 / reveal_tx.vsize as f64;

        let target_fee_rate = parent_fee_rate * 2.0;
        let cpfp_txid = sequencer
            .client
            .http_client()
            .da_bump_transaction_fee_cpfp(Some(*parent_txid), target_fee_rate)
            .await?;

        // Wait for child transaction
        da.wait_mempool_len(3, None).await?;

        let cpfp_entry = da.get_mempool_entry(&cpfp_txid).await?;
        let cpfp_fee_rate = cpfp_entry.fees.base.to_sat() as f64 / cpfp_entry.vsize as f64;

        // Verify the child tx has higher fee rate to accomodate for child + parent
        assert!(cpfp_fee_rate >= target_fee_rate);

        // Verify that child spends from reveal tx and keeps a correct chain of utxo spending
        let cpfp_tx = da.get_raw_transaction(&cpfp_txid, None).await?;
        assert_eq!(cpfp_tx.input[0].previous_output.txid, *parent_txid);

        da.generate(1, None).await?;
        let hash = da.get_best_block_hash().await?;
        let block = da.get_block(&hash).await?;

        let txids = block
            .txdata
            .iter()
            .skip(1) // Skip coinbase
            .map(|tx| tx.compute_txid())
            .collect::<Vec<_>>();

        // Assert proper tx ordering
        assert_eq!(
            &txids,
            &[reveal_tx.prev_tx.unwrap(), *parent_txid, cpfp_txid]
        );

        da.generate(FINALITY_DEPTH - 1, None).await?;
        let finalized_height = da.get_finalized_height().await?;

        batch_prover
            .wait_for_l1_height(finalized_height, None)
            .await?;

        let commitments = batch_prover
            .client
            .http_client()
            .get_sequencer_commitments_on_slot_by_number(finalized_height)
            .await?
            .unwrap();

        assert_eq!(commitments.len(), 1);

        // Mine prover tx before re-generating seqcommitments lest it conflicts
        da.wait_mempool_len(2, None).await?;
        da.generate(1, None).await?;

        // Generate another seqcommitments to assert that it spends from cpfp output
        for _ in 0..sequencer.min_soft_confirmations_per_commitment() {
            sequencer.client.send_publish_batch_request().await?;
        }

        // Wait for seqcommitments txs to hit mempool
        da.wait_mempool_len(2, None).await?;
        da.generate(1, None).await?;
        let hash = da.get_best_block_hash().await?;
        let block = da.get_block(&hash).await?;

        // Assert that commit tx spend from cpfp output
        let commit_tx = &block.txdata[1];
        let reveal_tx = &block.txdata[2];

        assert_eq!(commit_tx.input[0].previous_output.txid, cpfp_txid);
        assert_eq!(
            reveal_tx.input[0].previous_output.txid,
            commit_tx.compute_txid()
        );

        Ok(())
    }
}

#[tokio::test]
async fn test_cpfp_fee_bump() -> Result<()> {
    TestCaseRunner::new(CpfpFeeBumpingTest)
        .set_citrea_path(get_citrea_path())
        .run()
        .await
}
