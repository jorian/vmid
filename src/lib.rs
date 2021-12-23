#[macro_use]
pub mod menu;
use std::fmt;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct Data {
    pub orderbook: Orderbook,
}

impl Data {
    pub fn new(orderbook: Orderbook) -> Self {
        Data { orderbook }
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

#[derive(Clone, Debug, EnumIter)]
pub enum Chain {
    VRSC,
    VRSCTEST,
    Mutt,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
