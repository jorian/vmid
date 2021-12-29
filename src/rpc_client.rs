use vrsc_rpc::{Auth, Client as VerusClient};

pub struct RpcClient {
    pub client: VerusClient,
}

impl RpcClient {
    pub fn new(name: &str) -> Self {
        if let Ok(client) = VerusClient::chain(name, Auth::ConfigFile) {
            RpcClient { client }
        } else {
            panic!("no client could be made")
        }
    }
}

#[derive(Debug)]
pub struct Order {
    pub order_type: OrderType,
    pub name: String,
    pub price: f64,
    pub txid: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OrderType {
    Bid,
    Ask,
}
