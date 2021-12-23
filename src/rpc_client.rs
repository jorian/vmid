use vrsc_rpc::{Auth, Client as VerusClient};

pub struct RpcClient {
    pub client: VerusClient,
}

impl RpcClient {
    pub fn new() -> Self {
        RpcClient {
            client: VerusClient::chain("vrsctest", Auth::ConfigFile).expect("a client"),
        }
    }
}

#[derive(Debug)]
pub struct Order {
    pub order_type: OrderType,
    pub name: String,
    pub price: f64,
}

#[derive(Debug)]
pub enum OrderType {
    Bid,
    Ask,
}
