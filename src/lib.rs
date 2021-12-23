#[macro_use]
pub mod menu;
mod util;

use std::fmt;

#[derive(Debug)]
pub struct Data {
    pub orderbook: Orderbook,
    pub local_chains: Vec<Chain>,
}

impl Data {
    pub fn new(orderbook: Orderbook) -> Self {
        Data {
            orderbook,
            local_chains: util::find_local_chain_installations(),
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
