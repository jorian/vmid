use color_eyre::Report;
use cursive::traits::*;
use cursive::views::{Dialog, ScrollView, TextView};
use tracing::*;
use tracing_subscriber::EnvFilter;
use vmid::menu;
use vrsc_rpc::json::identity::OfferVariant::*;
use vrsc_rpc::RpcApi;

fn main() {
    setup().expect("set up of reporting");
    info!("setup succeeded");
    let mut siv = cursive::default();

    menu::new(&mut siv);

    siv.set_user_data(vmid::data::Data::new());

    let dialog = Dialog::new()
        .content(TextView::new("Selected:"))
        .content(TextView::new("<none>").with_name("chain_name"))
        .button("fetch offers", |s| {
            let offers = s.with_user_data(|data: &mut vmid::data::Data| {
                let offers = data
                    .active_chain
                    .rpc_client
                    .client
                    .get_offers(&data.active_chain.chain.0, true, false)
                    .unwrap();
                debug!("{:#?}", offers);

                offers
            });

            let mut offercollection = vec![];
            if let Some(orderbook) = offers {
                for (_, offers) in orderbook {
                    for marketplace_offer in offers {
                        match marketplace_offer.offer.offer {
                            IdentityOffer(id_offer) => {
                                offercollection.push(vmid::rpc_client::Order {
                                    order_type: vmid::rpc_client::OrderType::Ask,
                                    name: id_offer.name,
                                    price: marketplace_offer.price,
                                })
                            }
                            _ => {}
                        }
                        match marketplace_offer.offer.accept {
                            IdentityOffer(id_offer) => {
                                offercollection.push(vmid::rpc_client::Order {
                                    order_type: vmid::rpc_client::OrderType::Bid,
                                    name: id_offer.name,
                                    price: marketplace_offer.price,
                                })
                            }
                            _ => {}
                        };
                    }
                }
            }
            s.pop_layer();
            s.add_layer(ScrollView::new(
                Dialog::new()
                    .content(TextView::new(&format!("{:#?}", offercollection)))
                    .button("Quit", |s| s.quit()),
            ))
        });

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
