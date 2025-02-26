//! Common RPC crate provides helper methods that are needed in rpc servers
use std::time::Duration;

use futures::future::BoxFuture;
use futures::FutureExt;
use hyper::Method;
use jsonrpsee::core::RegisterMethodError;
use jsonrpsee::server::middleware::http::ProxyGetRequestLayer;
use jsonrpsee::server::middleware::rpc::RpcServiceT;
use jsonrpsee::types::error::{INTERNAL_ERROR_CODE, INTERNAL_ERROR_MSG};
use jsonrpsee::types::{ErrorObjectOwned, Request};
use jsonrpsee::{MethodResponse, RpcModule};
use sov_db::ledger_db::{LedgerDB, SharedLedgerOps};
use sov_db::schema::types::SoftConfirmationNumber;
use tower_http::cors::{Any, CorsLayer};

// Exit early if head_batch_num is below this threshold
const BLOCK_NUM_THRESHOLD: u64 = 2;

/// Register the healthcheck rpc
pub fn register_healthcheck_rpc<T: Send + Sync + 'static>(
    rpc_methods: &mut RpcModule<T>,
    ledger_db: LedgerDB,
) -> Result<(), RegisterMethodError> {
    let mut rpc = RpcModule::new(ledger_db);

    rpc.register_async_method("health_check", |_, ledger_db, _| async move {
        let error = |msg: &str| {
            ErrorObjectOwned::owned(
                INTERNAL_ERROR_CODE,
                INTERNAL_ERROR_MSG,
                Some(msg.to_string()),
            )
        };

        let Some((SoftConfirmationNumber(head_batch_num), _)) = ledger_db
            .get_head_soft_confirmation()
            .map_err(|err| error(&format!("Failed to get head soft batch: {}", err)))?
        else {
            return Ok::<(), ErrorObjectOwned>(());
        };

        // TODO: if the first blocks are not being produced properly, this might cause healthcheck to always return Ok
        if head_batch_num < BLOCK_NUM_THRESHOLD {
            return Ok::<(), ErrorObjectOwned>(());
        }

        let soft_batches = ledger_db
            .get_soft_confirmation_range(
                &(SoftConfirmationNumber(head_batch_num - 1)
                    ..=SoftConfirmationNumber(head_batch_num)),
            )
            .map_err(|err| error(&format!("Failed to get soft batch range: {}", err)))?;

        let block_time_s = (soft_batches[1].timestamp - soft_batches[0].timestamp).max(1);
        tokio::time::sleep(Duration::from_millis(block_time_s * 1500)).await;

        let (new_head_batch_num, _) = ledger_db
            .get_head_soft_confirmation()
            .map_err(|err| error(&format!("Failed to get head soft batch: {}", err)))?
            .unwrap();
        if new_head_batch_num > SoftConfirmationNumber(head_batch_num) {
            Ok::<(), ErrorObjectOwned>(())
        } else {
            Err(error("Block number is not increasing"))
        }
    })?;

    rpc_methods.merge(rpc)
}

/// Returns health check proxy layer to be used as http middleware
pub fn get_healthcheck_proxy_layer() -> ProxyGetRequestLayer {
    ProxyGetRequestLayer::new("/health", "health_check").unwrap()
}

/// Returns cors layer to be used as http middleware
pub fn get_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any)
}

#[derive(Debug, Clone)]
pub struct Logger<S>(pub S);

impl<'a, S> RpcServiceT<'a> for Logger<S>
where
    S: RpcServiceT<'a> + Send + Sync + Clone + 'a,
{
    type Future = BoxFuture<'a, MethodResponse>;

    fn call(&self, req: Request<'a>) -> Self::Future {
        let req_id = req.id();
        let req_method = req.method_name().to_string();

        tracing::debug!(id = ?req_id, method = ?req_method, params = ?req.params().as_str(), "rpc_request");

        let service = self.0.clone();
        async move {
            let resp = service.call(req).await;
            if resp.is_success() {
                tracing::debug!(id = ?req_id, method = ?req_method, result = ?resp.as_result(), "rpc_success");
            } else {
                tracing::warn!(id = ?req_id, method = ?req_method, result = ?resp.as_result(), "rpc_error");
            }

            resp
        }
        .boxed()
    }
}
