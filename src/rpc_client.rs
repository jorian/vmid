use std::str::FromStr;
use vrsc_rpc::{json::identity::OfferVariant::*, Auth, Client as VerusClient, RpcApi};

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

    pub(crate) fn get_currency_offers(&self, currency: &str) -> Vec<Order> {
        let offers = self.client.get_offers(&currency, true, false);
        let mut offercollection = vec![];
        if let Ok(orderbook) = offers {
            for (_, offers) in orderbook {
                for marketplace_offer in offers {
                    match marketplace_offer.offer.offer {
                        IdentityOffer(id_offer) => offercollection.push(Order {
                            order_type: OrderType::Ask,
                            name: id_offer.name,
                            price: marketplace_offer.price,
                            txid: marketplace_offer.offer.txid.to_string(),
                            expiry: marketplace_offer.offer.blockexpiry,
                        }),
                        _ => {}
                    }
                    match marketplace_offer.offer.accept {
                        IdentityOffer(id_offer) => offercollection.push(Order {
                            order_type: OrderType::Bid,
                            name: id_offer.name,
                            price: marketplace_offer.price,
                            txid: marketplace_offer.offer.txid.to_string(),
                            expiry: marketplace_offer.offer.blockexpiry,
                        }),
                        _ => {}
                    };
                }
            }
        }

        offercollection
    }
}

#[derive(Debug)]
pub struct Order {
    pub order_type: OrderType,
    pub name: String,
    pub price: f64,
    pub txid: String,
    pub expiry: u64,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OrderType {
    Bid,
    Ask,
}

impl FromStr for OrderType {
    type Err = ();
    fn from_str(input: &str) -> Result<OrderType, Self::Err> {
        match input {
            "asks" => Ok(OrderType::Ask),
            "bids" => Ok(OrderType::Bid),
            _ => Err(()),
        }
    }
}
