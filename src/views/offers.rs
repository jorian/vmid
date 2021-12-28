use crate::UserData;
use cursive::{
    traits::*,
    view::IntoBoxedView,
    views::{Dialog, ScrollView, TextView},
    Cursive, View,
};
use std::sync::Arc;
use tracing::debug;
use vrsc_rpc::json::identity::OfferVariant::*;
use vrsc_rpc::RpcApi;

pub fn new() -> Box<dyn View> {
    Dialog::new()
        .content(TextView::new("Selected:"))
        .content(TextView::new("<none>").with_name("chain_name"))
        .button("fetch offers", fetch_offers)
        .into_boxed_view()
}

fn fetch_offers(s: &mut Cursive) {
    let cb_sink = s.cb_sink().clone();
    let data = Arc::clone(&s.user_data::<UserData>().unwrap());
    debug!("orders being fetched");
    std::thread::spawn(move || {
        debug!("order fetch thread started");
        let offers = {
            let name = data.lock().unwrap().active_chain.name.clone();
            debug!("orders to fetch for: {}", &name);
            data.lock()
                .unwrap()
                .active_chain
                .rpc_client
                .client
                .get_offers(&name, true, false)
        };
        debug!("{:#?}", offers);

        let mut offercollection = vec![];
        if let Ok(orderbook) = offers {
            for (_, offers) in orderbook {
                for marketplace_offer in offers {
                    match marketplace_offer.offer.offer {
                        IdentityOffer(id_offer) => offercollection.push(crate::rpc_client::Order {
                            order_type: crate::rpc_client::OrderType::Ask,
                            name: id_offer.name,
                            price: marketplace_offer.price,
                        }),
                        _ => {}
                    }
                    match marketplace_offer.offer.accept {
                        IdentityOffer(id_offer) => offercollection.push(crate::rpc_client::Order {
                            order_type: crate::rpc_client::OrderType::Bid,
                            name: id_offer.name,
                            price: marketplace_offer.price,
                        }),
                        _ => {}
                    };
                }
            }
        }
        cb_sink
            .send(Box::new(move |s: &mut Cursive| {
                s.pop_layer();
                s.add_layer(ScrollView::new(
                    Dialog::new()
                        .content(TextView::new(&format!("{:#?}", offercollection)))
                        .button("fetch offers", fetch_offers),
                ))
            }))
            .unwrap();
    });
}
