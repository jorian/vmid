use super::rpc_client::RpcClient;
use std::fmt;
use tracing::*;
pub struct Data {
    pub active_chain: ActiveChain,
    pub local_chains: Vec<Chain>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            active_chain: ActiveChain::new(Chain(String::from(""))),
            local_chains: vec![],
        }
    }
}

// #[derive(Debug)]
pub struct ActiveChain {
    pub chain: Chain,
    pub rpc_client: RpcClient,
}

impl ActiveChain {
    pub fn new(chain: Chain) -> Self {
        let name = chain.clone();
        debug!("{:?}", &name);
        ActiveChain {
            chain,
            rpc_client: RpcClient::new(&name.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Chain(pub String);

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
