use crate::{rpc_client::OrderType, UserData};
use cursive::{
    align::HAlign,
    traits::*,
    view::IntoBoxedView,
    views::{Button, Dialog, DummyView, LinearLayout, NamedView, Panel, TextView},
    Cursive, View,
};
use cursive_aligned_view::Alignable;
use cursive_table_view::{TableView, TableViewItem};
use std::{cmp::Ordering, str::FromStr, sync::Arc};
use tracing::*;
use vrsc_rpc::RpcApi;

pub fn new<S: Into<String>>(title: S) -> Box<dyn View> {
    Panel::new(
        LinearLayout::vertical()
            .child(
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
                                .child(TextView::new("Asks").align_center())
                                .child(create_table(OfferType::Ask).full_height()),
                        )
                        .full_width(),
                    )
                    .fixed_height(25),
            )
            .child(
                LinearLayout::horizontal()
                    .child(Button::new("fetch", populate_with_orders))
                    .child(DummyView {}.fixed_width(2))
                    .child(Button::new("quit", |s| s.quit()))
                    .align_center(),
            ),
    )
    .title(title.into())
    .with_name("orderbook_panel")
    .into_boxed_view()
}

fn create_table(offer_type: OfferType) -> NamedView<TableView<OfferRow, OffersColumn>> {
    fn internal_table(table_name: String) -> NamedView<TableView<OfferRow, OffersColumn>> {
        let table_view = TableView::<OfferRow, OffersColumn>::new()
            .column(OffersColumn::Name, "Name", |c| c.width_percent(70))
            .column(OffersColumn::Price, "Price", |c| {
                c.ordering(Ordering::Greater)
                    .align(HAlign::Right)
                    .width_percent(30)
            });

        let table_name_c = table_name.clone();

        table_view
            .on_submit(move |siv, _row, index| {
                let item = siv.call_on_name(
                    &table_name_c,
                    |table_view: &mut TableView<OfferRow, OffersColumn>| {
                        table_view.borrow_item(index).unwrap().clone()
                    },
                );

                let active_chain = siv
                    .user_data::<UserData>()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .active_chain
                    .clone();

                let block_height = active_chain.rpc_client.client.get_block_count().unwrap();

                if let Some(item) = item {
                    siv.add_layer(
                        Dialog::new()
                            .content(TextView::new(format!(
                                "txid: {}\nprice: {} {}\nexpires in {} blocks",
                                item.txid.clone(),
                                item.price,
                                &item.currency,
                                item.expiry - block_height as u64
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
        OfferType::Ask => return internal_table(String::from("asks")),
        OfferType::Bid => return internal_table(String::from("bids")),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum OffersColumn {
    Name,
    Price,
}

#[derive(Clone, Debug)]
struct OfferRow {
    currency: String,
    name: String,
    price: f64,
    txid: String,
    expiry: u64,
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

fn populate_with_orders(s: &mut Cursive) {
    let cb_sink = s.cb_sink().clone();
    let data = Arc::clone(&s.user_data::<UserData>().unwrap());
    debug!("orders being fetched");
    std::thread::spawn(move || {
        debug!("order fetch thread started");
        let currency = data.lock().unwrap().active_chain.name.clone();
        let offercollection = {
            debug!("orders to fetch for: {}", &currency);
            data.lock()
                .unwrap()
                .active_chain
                .rpc_client
                .get_currency_offers(&currency)
        };

        let currency_c = currency.clone();
        cb_sink
            .send(Box::new(move |s: &mut Cursive| {
                for order_type in &["bids", "asks"] {
                    s.call_on_name(
                        order_type,
                        |table: &mut TableView<OfferRow, OffersColumn>| {
                            table.clear();
                            table.set_items({
                                let mut table = offercollection
                                    .iter()
                                    .filter(|item| {
                                        item.order_type == OrderType::from_str(order_type).unwrap()
                                    })
                                    .map(|item| OfferRow {
                                        currency: currency_c.clone(),
                                        name: item.name.clone(),
                                        price: item.price,
                                        txid: item.txid.clone(),
                                        expiry: item.expiry,
                                    })
                                    .collect::<Vec<OfferRow>>();
                                table.sort_by(|a, b| a.name.cmp(&b.name));

                                table
                            });
                            table.set_selected_item(0);
                        },
                    );
                }
            }))
            .unwrap();
    });
}
