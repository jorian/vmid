use super::rpc_client::RpcClient;
use std::fmt;
use tracing::*;
pub struct Data {
    pub active_chain: Chain,
    pub local_chains: Vec<Chain>,
}

impl Data {
    pub fn new() -> Self {
        let local_chains = crate::util::find_local_chain_installations();
        Data {
            active_chain: Chain::new(""),
            local_chains: crate::util::find_local_chain_installations(),
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
        debug!("new Chain.{:?}", &name);
        Chain { name, rpc_client }
    }
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
