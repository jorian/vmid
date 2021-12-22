pub mod menu;

pub struct Orderbook {
    chain: Chain,
}

impl Orderbook {
    pub fn new(chain: Chain) -> Self {
        Orderbook { chain }
    }
}

#[derive(Debug)]
pub enum Chain {
    VRSC,
    VRSCTEST,
    Mutt,
}
