use crate::{rpc_client::OrderType, UserData};
use cursive::{
    align::HAlign,
    traits::*,
    view::IntoBoxedView,
    views::{Button, Dialog, LinearLayout, NamedView, Panel, TextView},
    Cursive, View,
};
use cursive_aligned_view::Alignable;
use cursive_table_view::{TableView, TableViewItem};
use std::{cmp::Ordering, sync::Arc};
use tracing::*;
use vrsc_rpc::json::identity::OfferVariant::*;
use vrsc_rpc::RpcApi;

pub fn new<S: Into<String>>(title: S) -> Box<dyn View> {
    Panel::new(
        LinearLayout::horizontal()
            .child(
                Panel::new(
                    LinearLayout::vertical()
                        .child(TextView::new("Bids").align_center())
                        .child(create_table(OfferType::Bid).full_height()),
                )
                .full_width(),
            )
            .child(
                Panel::new(
                    LinearLayout::vertical()
                        .child(Button::new("FETCH", fetch_offers).align_center()),
                )
                .full_width(),
            )
            .child(
                Panel::new(
                    LinearLayout::vertical()
                        .child(TextView::new("Asks").align_center())
                        .child(create_table(OfferType::Ask).full_height()),
                )
                .full_width(),
            )
            .fixed_height(25),
    )
    .title(title.into())
    .with_name("orderbook_panel")
    .into_boxed_view()
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum OffersColumn {
    Name,
    Price,
}

#[derive(Clone, Debug)]
struct OfferRow {
    name: String,
    price: f64,
    txid: String,
}

enum OfferType {
    Bid,
    Ask,
}

impl TableViewItem<OffersColumn> for OfferRow {
    fn to_column(&self, column: OffersColumn) -> String {
        match column {
            OffersColumn::Name => self.name.to_string(),
            OffersColumn::Price => format!("{}", self.price),
        }
    }

    fn cmp(&self, other: &Self, column: OffersColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            OffersColumn::Name => self.name.cmp(&other.name),
            OffersColumn::Price => self.price.partial_cmp(&other.price).unwrap(),
        }
    }
}

fn create_table(offer_type: OfferType) -> NamedView<TableView<OfferRow, OffersColumn>> {
    fn internal_table<S: Into<String>>(
        table_name: S,
    ) -> NamedView<TableView<OfferRow, OffersColumn>> {
        let table_view = TableView::<OfferRow, OffersColumn>::new()
            .column(OffersColumn::Name, "Name", |c| c.width_percent(50))
            .column(OffersColumn::Price, "Price", |c| {
                c.ordering(Ordering::Greater)
                    .align(HAlign::Right)
                    .width_percent(50)
            });

        let table_name = table_name.into();
        let table_name_c = table_name.clone();

        table_view
            .on_submit(move |siv, _row, index| {
                let item = siv.call_on_name(
                    &table_name_c,
                    |table_view: &mut TableView<OfferRow, OffersColumn>| {
                        table_view.borrow_item(index).unwrap().clone()
                    },
                );

                if let Some(item) = item {
                    siv.add_layer(
                        Dialog::new()
                            .content(TextView::new(format!(
                                "txid: {}\nprice: {}",
                                item.txid.clone(),
                                item.price
                            )))
                            .button("close", |s| {
                                s.pop_layer();
                            })
                            .button("take offer", |_| {}),
                    )
                };
            })
            .with_name(&table_name)
    }

    match offer_type {
        OfferType::Ask => return internal_table("asks"),
        OfferType::Bid => return internal_table("bids"),
    }
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
                            txid: marketplace_offer.offer.txid.to_string(),
                            expiry: marketplace_offer.offer.blockexpiry,
                        }),
                        _ => {}
                    }
                    match marketplace_offer.offer.accept {
                        IdentityOffer(id_offer) => offercollection.push(crate::rpc_client::Order {
                            order_type: crate::rpc_client::OrderType::Bid,
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
        cb_sink
            .send(Box::new(move |s: &mut Cursive| {
                s.call_on_name("bids", |table: &mut TableView<OfferRow, OffersColumn>| {
                    table.clear();
                    table.set_items({
                        let mut table = offercollection
                            .iter()
                            .filter(|item| item.order_type == OrderType::Bid)
                            .map(|item| OfferRow {
                                name: item.name.clone(),
                                price: item.price,
                                txid: item.txid.clone(),
                            })
                            .collect::<Vec<OfferRow>>();
                        table.sort_by(|a, b| a.name.cmp(&b.name));

                        table
                    });
                    table.set_selected_item(0);
                });

                s.call_on_name("asks", |table: &mut TableView<OfferRow, OffersColumn>| {
                    table.clear();
                    table.set_items({
                        let mut table = offercollection
                            .iter()
                            .filter(|item| item.order_type == OrderType::Ask)
                            .map(|item| OfferRow {
                                name: item.name.clone(),
                                price: item.price,
                                txid: item.txid.clone(),
                            })
                            .collect::<Vec<OfferRow>>();
                        table.sort_by(|a, b| a.name.cmp(&b.name));

                        table
                    });
                    // table.sort_by(OffersColumn::Name, Ordering::Greater);
                    // table.order();
                    table.set_selected_item(0);
                });
            }))
            .unwrap();
    });
}
