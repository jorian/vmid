use super::rpc_client::RpcClient;
use std::{fmt, sync::Arc};
use tracing::*;
pub struct Data {
    pub active_chain: Arc<Chain>,
    pub local_chains: Arc<Vec<Arc<Chain>>>,
}

impl Data {
    pub fn new() -> Self {
        let local_chains = crate::util::find_local_chain_installations();
        if let Some(first) = local_chains.first() {
            Data {
                active_chain: Arc::clone(first),
                local_chains: local_chains,
            }
        } else {
            panic!("no installations found")
        }
    }
}

pub struct Chain {
    pub name: String,
    pub rpc_client: RpcClient,
}

impl Chain {
    pub fn new<S: Into<String>>(chain: S) -> Self {
        let name = chain.into();
        let rpc_client = RpcClient::new(&name);
        info!("new chain: {}", &name);
        Chain { name, rpc_client }
    }
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Chain").field("name", &self.name).finish()
    }
}
