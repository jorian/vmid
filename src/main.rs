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
                    .rpc_client
                    .client
                    .get_offers(&data.orderbook.chain.0, true, false)
                    .unwrap();
                debug!("{:#?}", offers);

                offers
            });

            if let Some(orderbook) = offers {
                let mut offercollection = vec![];
                for (_, offers) in orderbook {
                    for offer in offers {
                        offercollection.push(match offer.offer.offer {
                            CurrencyOffer(mut map) => {
                                let (name, price) =
                                    map.drain().next().expect("a i-address and a price");
                                vmid::rpc_client::Order { name, price }
                            }
                            IdentityOffer(id_offer) => vmid::rpc_client::Order {
                                name: id_offer.name,
                                price: offer.price,
                            },
                        });
                    }
                }
            }
            // s.add_layer(ScrollView::new().content(TextView::new(&format!("{:#?}", offers))))
            s.add_layer(ScrollView::new(
                Dialog::new().content(TextView::new(&format!("{:#?}", "test"))),
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
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
