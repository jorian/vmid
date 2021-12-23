use super::rpc_client::RpcClient;
use std::fmt;

pub struct Data {
    pub orderbook: Orderbook,
    pub local_chains: Vec<Chain>,
    pub rpc_client: RpcClient,
}

impl Data {
    pub fn new() -> Self {
        Data {
            orderbook: Orderbook::new(Chain(String::from(""))),
            local_chains: vec![],
            rpc_client: RpcClient::new(),
        }
    }
}

#[derive(Debug)]
pub struct Orderbook {
    pub chain: Chain,
}

impl Orderbook {
    pub fn new(chain: Chain) -> Self {
        Orderbook { chain }
    }
}

#[derive(Clone, Debug)]
pub struct Chain(pub String);

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
