use super::rpc_client::RpcClient;
use std::{cell::RefCell, fmt, rc::Rc};
use tracing::*;
pub struct Data {
    pub active_chain: Rc<RefCell<Chain>>,
    pub local_chains: Rc<Vec<Rc<RefCell<Chain>>>>,
}

impl Data {
    pub fn new() -> Self {
        let local_chains = crate::util::find_local_chain_installations();
        if let Some(first) = local_chains.first() {
            Data {
                active_chain: Rc::clone(first),
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

impl std::fmt::Debug for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Chain").field("name", &self.name).finish()
    }
}
