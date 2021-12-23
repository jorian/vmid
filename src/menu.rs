use cursive::menu;
use cursive::traits::*;
use cursive::views::Dialog;
use cursive::Cursive;
use strum::IntoEnumIterator;
use tracing::{error, info};

use crate::Chain;

pub fn new(siv: &mut Cursive) {
    siv.menubar()
        .add_subtree("File", menu::MenuTree::new().leaf("Quit", |s| s.quit()))
        .add_subtree(
            "View",
            menu::MenuTree::new()
                .leaf("All offers", |_| {})
                .leaf("My offers", |_| {})
                .leaf("Currency offers", |_| {})
                .leaf("Identity offers", |_| {}),
        )
        .add_subtree(
            "Select",
            menu::MenuTree::new().with(|tree| {
                for chain in Chain::iter() {
                    tree.add_leaf(chain.to_string(), move |s| {
                        set_active_chain(s, chain.clone())
                    });
                }
            }),
        )
        .add_subtree(
            "New",
            menu::MenuTree::new()
                .leaf("Offer", |_| {})
                .leaf("Identity", |_| {})
                .leaf("Donation", |_| {}),
        )
        .add_delimiter()
        .add_leaf(format!("Search {}", '\u{1F50D}'), |s| {
            s.add_layer(Dialog::info("Search here"))
        })
        .add_leaf("Help", |_| {});

    siv.set_autohide_menu(false);
}

fn set_active_chain(s: &mut Cursive, chain: Chain) {
    info!("{} selected", &chain);
    s.call_on_name("chain_name", |view: &mut cursive::views::TextView| {
        view.set_content(&chain.to_string())
    });
    if let None = s.with_user_data(|data: &mut crate::Data| data.orderbook.chain = chain) {
        error!("user data was not updated, undefined behaviour");
        panic!("user_data not set");
    }
}
