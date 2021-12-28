use color_eyre::Report;
use cursive::views::{Dialog, ScrollView, TextView};
use cursive::{traits::*, Cursive};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::*;
use tracing_subscriber::EnvFilter;
use vmid::menu;
use vrsc_rpc::json::identity::OfferVariant::*;
use vrsc_rpc::RpcApi;

type UserData = Arc<Mutex<vmid::data::Data>>;
fn main() {
    setup().expect("set up of reporting");
    info!("setup succeeded");
    let mut siv = cursive::default();

    let data = Arc::new(Mutex::new(vmid::data::Data::new()));
    menu::new(&mut siv, data.clone());

    siv.set_user_data(data.clone());

    let dialog = Dialog::new()
        .content(TextView::new("Selected:"))
        .content(TextView::new("<none>").with_name("chain_name"))
        .button("fetch offers", fetch_offers);

    siv.add_layer(dialog);

    siv.run();
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

fn fetch_offers(s: &mut Cursive) {
    let cb_sink = s.cb_sink().clone();
    let data = Arc::clone(&s.user_data::<UserData>().unwrap());
    debug!("orders being fetched");
    std::thread::spawn(move || {
        debug!("thread started");
        let offers = {
            debug!("fetch some orders");
            let name = data
                .lock()
                .unwrap()
                .active_chain
                .lock()
                .unwrap()
                .name
                .clone();
            debug!("{}", &name);
            data.lock()
                .unwrap()
                .active_chain
                .lock()
                .unwrap()
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
                        IdentityOffer(id_offer) => offercollection.push(vmid::rpc_client::Order {
                            order_type: vmid::rpc_client::OrderType::Ask,
                            name: id_offer.name,
                            price: marketplace_offer.price,
                        }),
                        _ => {}
                    }
                    match marketplace_offer.offer.accept {
                        IdentityOffer(id_offer) => offercollection.push(vmid::rpc_client::Order {
                            order_type: vmid::rpc_client::OrderType::Bid,
                            name: id_offer.name,
                            price: marketplace_offer.price,
                        }),
                        _ => {}
                    };
                }
            }
        }
        std::thread::sleep(Duration::from_secs(5));
        cb_sink
            .send(Box::new(move |s: &mut Cursive| {
                s.pop_layer();
                s.add_layer(ScrollView::new(
                    Dialog::new()
                        .content(TextView::new(&format!("{:#?}", offercollection)))
                        .button("Fetch", fetch_offers),
                ))
            }))
            .unwrap();
    });
}
