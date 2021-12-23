use vrsc_rpc::{Auth, Client as VerusClient, RpcApi as VerusRpcApi};

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

pub struct Order {
    pub name: String,
    pub price: f64,
}
